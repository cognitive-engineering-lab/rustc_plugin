use std::{
  cell::RefCell, collections::hash_map::Entry, default::Default, ffi::OsStr,
  path::PathBuf,
};

use anyhow::{bail, ensure, Context, Result};
use rustc_data_structures::{fx::FxHashMap as HashMap, sync::Lrc};
use rustc_hir::{
  intravisit::{self, Visitor},
  BodyId,
};
use rustc_index::IndexVec;
use rustc_middle::ty::TyCtxt;
use rustc_span::{source_map::SourceMap, FileName, RealFileName, SourceFile, Span};
#[cfg(feature = "serde")]
use serde::Serialize;
#[cfg(feature = "ts-rs")]
use ts_rs::TS;

use super::filename::{Filename, FilenameIndex};
use crate::cache::Cache;

struct CharByteMapping {
  byte_to_char: HashMap<BytePos, CharPos>,
  char_to_byte: HashMap<CharPos, BytePos>,
}

impl CharByteMapping {
  pub fn build(file: &SourceFile) -> Self {
    let mut byte_to_char = HashMap::default();
    let mut char_to_byte = HashMap::default();

    macro_rules! check_insert {
      ($bpos:expr, $cpos:expr) => {
        let _r = byte_to_char.insert($bpos, $cpos);
        debug_assert!(
          _r.is_none(),
          "byte_to_char: bytepos={:?}, charpos={:?}",
          $bpos,
          $cpos
        );
        let _r = char_to_byte.insert($cpos, $bpos);
        debug_assert!(
          _r.is_none(),
          "char_to_byte: bytepos={:?}, charpos={:?}",
          $bpos,
          $cpos
        );
      };
    }

    for line in 0 .. file.count_lines() {
      let line_str = file.get_line(line).unwrap();
      let line_bounds = file.line_bounds(line);
      let line_start = line_bounds.start.0 as usize;
      let mut last_column = 0;
      let mut last_offset = 0;
      for (column, (byte_offset, c)) in line_str.char_indices().enumerate() {
        let bpos = BytePos(line_start + byte_offset);
        let cpos = CharPos { line, column };
        check_insert!(bpos, cpos);
        last_column = column + 1;
        last_offset = byte_offset + c.len_utf8();
      }

      let bpos = BytePos(line_start + last_offset);
      let cpos = CharPos {
        line,
        column: last_column,
      };
      check_insert!(bpos, cpos);
    }

    CharByteMapping {
      byte_to_char,
      char_to_byte,
    }
  }

  #[allow(unused)]
  pub fn byte_to_char(&self, pos: BytePos) -> CharPos {
    *self
      .byte_to_char
      .get(&pos)
      .unwrap_or_else(|| panic!("Could not find char pos for {pos:?}"))
  }

  pub fn char_to_byte(&self, pos: CharPos) -> BytePos {
    *self
      .char_to_byte
      .get(&pos)
      .unwrap_or_else(|| panic!("Could not find byte pos for {pos:?}"))
  }
}

#[derive(Default)]
pub struct RangeContext {
  filenames: IndexVec<FilenameIndex, Filename>,
  path_mapping: HashMap<FilenameIndex, Lrc<SourceFile>>,
  char_byte_mapping: Cache<FilenameIndex, CharByteMapping>,
}

thread_local! {
  static CONTEXT: RefCell<RangeContext> = RefCell::new(RangeContext::default());
}

impl Filename {
  fn intern_with_ctx(self, ctx: &mut RangeContext) -> FilenameIndex {
    let existing = ctx.filenames.iter_enumerated().find(|(_, f)| &self == *f);
    match existing {
      Some((index, _)) => index,
      None => ctx.filenames.push(self),
    }
  }

  pub fn intern<T: ?Sized + AsRef<OsStr>>(t: &T) -> FilenameIndex {
    let filename = Filename(PathBuf::from(t));
    CONTEXT.with(|ctx| filename.intern_with_ctx(&mut ctx.borrow_mut()))
  }
}

impl FilenameIndex {
  pub fn find_source_file(self, source_map: &SourceMap) -> Result<Lrc<SourceFile>> {
    CONTEXT.with(|ctx| {
      let ctx = &mut *ctx.borrow_mut();
      match ctx.path_mapping.entry(self) {
        Entry::Occupied(entry) => Ok(Lrc::clone(entry.get())),
        Entry::Vacant(entry) => {
          let files = source_map.files();
          ensure!(
            ctx.filenames.get(self).is_some(),
            "Missing file index!"
          );
          let filename = &ctx.filenames[self];
          let filename = filename.0
            .canonicalize()
            .unwrap_or_else(|_| filename.0.to_path_buf());
          let rustc_filename = files
            .iter()
            .map(|file| &file.name)
            .find(|name| match &name {
              // rustc seems to store relative paths to files in the workspace, so if filename is absolute,
              // we can compare them using Path::ends_with
              FileName::Real(RealFileName::LocalPath(other)) => {
                let canonical = other.canonicalize();
                let other = canonical.as_ref().unwrap_or(other);
                filename.ends_with(other)
              }
              _ => false,
            })
            .with_context(|| {
              format!(
                "Could not find SourceFile for path: {}. Available SourceFiles were: [{}]",
                filename.display(),
                files
                  .iter()
                  .filter_map(|file| match &file.name {
                    FileName::Real(RealFileName::LocalPath(other)) =>
                      Some(format!("{}", other.display())),
                    _ => None,
                  })
                  .collect::<Vec<_>>()
                  .join(", ")
              )
            })?;
          let file = source_map.get_source_file(rustc_filename).unwrap();
          entry.insert(Lrc::clone(&file));
          Ok(file)
        }
      }
    })
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "ts-rs", derive(TS))]
pub struct BytePos(pub usize);

/// CharPos is designed to match VSCode's vscode.Position type.
/// Both line and column are 0-based.
///
/// A previous version of CharPos used a global character-based index,
/// naively thinking this was the same as VSCode's notion of an "offset".
/// However, for files using CRLF line endings, VSCode ignores the `\r`
/// when computing offsets, while Rustc does not. The unhappy compromise
/// is to use line-column as a common coordinate system, which is robust
/// to choice of line endings.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "ts-rs", derive(TS))]
pub struct CharPos {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "ts-rs", derive(TS))]
pub struct ByteRange {
  pub start: BytePos,
  pub end: BytePos,
  pub filename: FilenameIndex,
}

/// Data structure for sharing spans outside rustc.
///
/// Rustc uses byte indexes to describe ranges of source code, whereas
/// most Javascript-based editors I've encountered (e.g. VSCode) use
/// character-based (really grapheme-based) indexes. This data structure
/// along with [`ByteRange`] helps convert between the two representations.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "ts-rs", derive(TS))]
pub struct CharRange {
  pub start: CharPos,
  pub end: CharPos,
  pub filename: FilenameIndex,
}

impl ByteRange {
  pub fn as_char_range(&self, source_map: &SourceMap) -> CharRange {
    let file = self.filename.find_source_file(source_map).unwrap();

    CONTEXT.with(|ctx| {
      let ctx = ctx.borrow();
      let mapping: &CharByteMapping = ctx
        .char_byte_mapping
        .get(self.filename, |_| CharByteMapping::build(&file));

      let char_start = mapping.byte_to_char(self.start);
      let char_end = mapping.byte_to_char(self.end);

      CharRange {
        start: char_start,
        end: char_end,
        filename: self.filename,
      }
    })
  }

  pub fn from_char_range(
    char_start: CharPos,
    char_end: CharPos,
    filename: FilenameIndex,
    source_map: &SourceMap,
  ) -> Result<ByteRange> {
    let file = filename.find_source_file(source_map)?;

    CONTEXT.with(|ctx| {
      let ctx = ctx.borrow();
      let mapping = ctx
        .char_byte_mapping
        .get(filename, |_| CharByteMapping::build(&file));
      let byte_start = mapping.char_to_byte(char_start);
      let byte_end = mapping.char_to_byte(char_end);
      Ok(ByteRange {
        start: byte_start,
        end: byte_end,
        filename,
      })
    })
  }

  pub fn from_span(span: Span, source_map: &SourceMap) -> Result<Self> {
    CONTEXT.with(|ctx| {
      let mut ctx = ctx.borrow_mut();

      log::trace!("Converting to range: {span:?}");
      let file = source_map.lookup_source_file(span.lo());
      let filename = match &file.name {
        FileName::Real(RealFileName::LocalPath(filename)) => {
          Filename(filename.clone()).intern_with_ctx(&mut ctx)
        }
        filename => bail!("Range::from_span doesn't support {filename:?}"),
      };

      ensure!(
        source_map.ensure_source_file_source_present(file.clone()),
        "Could not load source for file: {:?}",
        file.name
      );
      let external = file.external_src.borrow();
      let _src = file
        .src
        .as_ref()
        .unwrap_or_else(|| external.get_source().as_ref().unwrap());

      let byte_start = BytePos(span.lo().0 as usize);
      let byte_end = BytePos(span.hi().0 as usize);

      Ok(ByteRange {
        start: byte_start,
        end: byte_end,
        filename,
      })
    })
  }

  pub fn substr(&self, s: &str) -> String {
    s[self.start.0 .. self.end.0].to_string()
  }
}

impl CharRange {
  pub fn from_span(span: Span, source_map: &SourceMap) -> Result<Self> {
    let byte_range = ByteRange::from_span(span, source_map)?;
    Ok(byte_range.as_char_range(source_map))
  }
}

/// Used to convert objects into a [`Span`] with access to [`TyCtxt`]
pub trait ToSpan {
  fn to_span(&self, tcx: TyCtxt) -> Result<Span>;
}

impl ToSpan for ByteRange {
  fn to_span(&self, _tcx: TyCtxt) -> Result<Span> {
    Ok(Span::with_root_ctxt(
      rustc_span::BytePos(self.start.0 as u32),
      rustc_span::BytePos(self.end.0 as u32),
    ))
  }
}

impl ToSpan for CharRange {
  fn to_span(&self, tcx: TyCtxt) -> Result<Span> {
    let range = ByteRange::from_char_range(
      self.start,
      self.end,
      self.filename,
      tcx.sess.source_map(),
    )?;
    range.to_span(tcx)
  }
}

fn qpath_to_span(tcx: TyCtxt, qpath: String) -> Result<Span> {
  struct Finder<'tcx> {
    tcx: TyCtxt<'tcx>,
    qpath: String,
    span: Option<Span>,
  }

  impl<'tcx> Visitor<'tcx> for Finder<'tcx> {
    fn visit_nested_body(&mut self, id: BodyId) {
      intravisit::walk_body(self, self.tcx.hir().body(id));

      let local_def_id = self.tcx.hir().body_owner_def_id(id);
      let function_path = self
        .tcx
        .def_path(local_def_id.to_def_id())
        .to_string_no_crate_verbose();
      if function_path[2 ..] == self.qpath {
        self.span = Some(self.tcx.hir().span(id.hir_id));
      }
    }
  }

  let mut finder = Finder {
    tcx,
    qpath,
    span: None,
  };
  tcx.hir().visit_all_item_likes_in_crate(&mut finder);
  finder
    .span
    .with_context(|| format!("No function with qpath {}", finder.qpath))
}

/// An externally-provided identifier of a function
pub enum FunctionIdentifier {
  /// Name of a function
  Qpath(String),

  /// Range of code possibly inside a function
  Range(CharRange),
}

impl ToSpan for FunctionIdentifier {
  fn to_span(&self, tcx: TyCtxt) -> Result<Span> {
    match self {
      FunctionIdentifier::Qpath(qpath) => qpath_to_span(tcx, qpath.clone()),
      FunctionIdentifier::Range(range) => range.to_span(tcx),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_utils;

  #[test]
  fn test_range() {
    let emoji = "🦀";
    let input = &format!(
      r#"fn main() {{
  let x = "{emoji}";
}}

// mysterious bytes
"#
    );

    test_utils::compile(input, |tcx| {
      let source_map = tcx.sess.source_map();
      let filename = Filename::intern("dummy.rs");
      filename.find_source_file(source_map).unwrap();

      let id = FunctionIdentifier::Qpath(String::from("main"));
      id.to_span(tcx).unwrap();

      let id = FunctionIdentifier::Qpath(String::from("foobar"));
      id.to_span(tcx).unwrap_err();

      let id = FunctionIdentifier::Range(CharRange {
        start: CharPos { line: 0, column: 0 },
        end: CharPos { line: 0, column: 1 },
        filename,
      });
      id.to_span(tcx).unwrap();

      let emoji_index = input.find(emoji).unwrap();
      let byte_range = ByteRange {
        start: BytePos(emoji_index),
        end: BytePos(emoji_index + emoji.len()),
        filename,
      };
      let char_range = byte_range.as_char_range(source_map);
      let emoji_line = 1;
      let emoji_column = 11;
      assert_eq!(char_range, CharRange {
        start: CharPos {
          line: emoji_line,
          column: emoji_column
        },
        end: CharPos {
          line: emoji_line,
          column: emoji_column + 1
        },
        filename
      });
    });
  }
}
