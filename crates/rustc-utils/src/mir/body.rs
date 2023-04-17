//! Utilities for [`Body`].

use std::{
  io::Write,
  path::Path,
  process::{Command, Stdio},
};

use anyhow::{ensure, Result};
use cfg_if::cfg_if;
use rustc_data_structures::{captures::Captures, fx::FxHashMap as HashMap};
use rustc_hir::{def_id::DefId, GeneratorKind, HirId};
use rustc_middle::{
  mir::{pretty::write_mir_fn, *},
  ty::{Region, Ty, TyCtxt},
};
use rustc_mir_dataflow::{fmt::DebugWithContext, Analysis, Results};
use smallvec::SmallVec;

use super::control_dependencies::ControlDependencies;
use crate::{PlaceExt, TyExt};

/// Extension trait for [`Body`].
pub trait BodyExt<'tcx> {
  type AllReturnsIter<'a>: Iterator<Item = Location>
  where
    Self: 'a;

  /// Returns an iterator over the locations of [`TerminatorKind::Return`] instructions in a body.
  fn all_returns(&self) -> Self::AllReturnsIter<'_>;

  type AllLocationsIter<'a>: Iterator<Item = Location>
  where
    Self: 'a;

  /// Returns an iterator over all the locations in a body.
  fn all_locations(&self) -> Self::AllLocationsIter<'_>;

  type LocationsIter: Iterator<Item = Location>;

  /// Returns all the locations in a [`BasicBlock`].
  fn locations_in_block(&self, block: BasicBlock) -> Self::LocationsIter;

  /// Returns a mapping from source-level variable names to [`Local`]s.
  fn debug_info_name_map(&self) -> HashMap<String, Local>;

  /// Converts a Body to a debug representation.
  fn to_string(&self, tcx: TyCtxt<'tcx>) -> Result<String>;

  /// Returns the [`HirId`] corresponding to a MIR [`Location`].
  ///
  /// You **MUST** use the `-Zmaximize-hir-to-mir-mapping` flag for this
  /// function to work.
  fn location_to_hir_id(&self, location: Location) -> HirId;

  fn source_info_to_hir_id(&self, info: &SourceInfo) -> HirId;

  /// Returns all the control dependencies within the CFG.
  ///
  /// See the [`control_dependencies`][super::control_dependencies] module documentation
  /// for details.
  fn control_dependencies(&self) -> ControlDependencies<BasicBlock>;

  /// If this body is an async function, then return the type of the context that holds
  /// locals across await calls.
  fn async_context(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<Ty<'tcx>>;

  type PlacesIter<'a>: Iterator<Item = Place<'tcx>>
  where
    Self: 'a;

  /// Returns an iterator over all projections of all local variables in the body.
  fn all_places(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Self::PlacesIter<'_>;

  type ArgRegionsIter<'a>: Iterator<Item = Region<'tcx>>
  where
    Self: 'a;

  /// Returns an iterator over all the regions that appear in argument types to the body.
  fn regions_in_args(&self) -> Self::ArgRegionsIter<'_>;

  type ReturnRegionsIter: Iterator<Item = Region<'tcx>>;

  /// Returns an iterator over all the regions that appear in the body's return type.
  fn regions_in_return(&self) -> Self::ReturnRegionsIter;

  /// Visualizes analysis results using graphviz/dot and writes them to
  /// a file in the `target/` directory named `<function name>.pdf`.
  fn write_analysis_results<A>(
    &self,
    results: &Results<'tcx, A>,
    def_id: DefId,
    tcx: TyCtxt<'tcx>,
  ) -> Result<()>
  where
    A: Analysis<'tcx>,
    A::Domain: DebugWithContext<A>;
}

impl<'tcx> BodyExt<'tcx> for Body<'tcx> {
  type AllReturnsIter<'a> = impl Iterator<Item = Location> + Captures<'tcx> + 'a where Self: 'a;
  fn all_returns(&self) -> Self::AllReturnsIter<'_> {
    self
      .basic_blocks
      .iter_enumerated()
      .filter_map(|(block, data)| match data.terminator().kind {
        TerminatorKind::Return => Some(Location {
          block,
          statement_index: data.statements.len(),
        }),
        _ => None,
      })
  }

  type AllLocationsIter<'a> = impl Iterator<Item = Location> + Captures<'tcx> + 'a where Self: 'a;
  fn all_locations(&self) -> Self::AllLocationsIter<'_> {
    self
      .basic_blocks
      .iter_enumerated()
      .flat_map(|(block, data)| {
        (0 .. data.statements.len() + 1).map(move |statement_index| Location {
          block,
          statement_index,
        })
      })
  }

  type LocationsIter = impl Iterator<Item = Location>;
  fn locations_in_block(&self, block: BasicBlock) -> Self::LocationsIter {
    let num_stmts = self.basic_blocks[block].statements.len();
    (0 ..= num_stmts).map(move |statement_index| Location {
      block,
      statement_index,
    })
  }

  fn debug_info_name_map(&self) -> HashMap<String, Local> {
    self
      .var_debug_info
      .iter()
      .filter_map(|info| match info.value {
        VarDebugInfoContents::Place(place) => Some((info.name.to_string(), place.local)),
        _ => None,
      })
      .collect()
  }

  fn to_string(&self, tcx: TyCtxt<'tcx>) -> Result<String> {
    let mut buffer = Vec::new();
    write_mir_fn(tcx, self, &mut |_, _| Ok(()), &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
  }

  fn location_to_hir_id(&self, location: Location) -> HirId {
    let source_info = self.source_info(location);
    self.source_info_to_hir_id(source_info)
  }

  fn source_info_to_hir_id(&self, info: &SourceInfo) -> HirId {
    let scope = &self.source_scopes[info.scope];
    let local_data = scope.local_data.as_ref().assert_crate_local();
    local_data.lint_root
  }

  fn control_dependencies(&self) -> ControlDependencies<BasicBlock> {
    ControlDependencies::build_many(
      &self.basic_blocks,
      self.all_returns().map(|loc| loc.block),
    )
  }

  fn async_context(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<Ty<'tcx>> {
    if matches!(tcx.generator_kind(def_id), Some(GeneratorKind::Async(..))) {
      Some(self.local_decls[Local::from_usize(2)].ty)
    } else {
      None
    }
  }

  type ArgRegionsIter<'a> = impl Iterator<Item = Region<'tcx>> + Captures<'tcx> + 'a
  where Self: 'a;

  type ReturnRegionsIter = impl Iterator<Item = Region<'tcx>>;

  type PlacesIter<'a> = impl Iterator<Item = Place<'tcx>> + Captures<'tcx> + 'a
  where Self: 'a;

  fn regions_in_args(&self) -> Self::ArgRegionsIter<'_> {
    self
      .args_iter()
      .flat_map(|arg_local| self.local_decls[arg_local].ty.inner_regions())
  }

  fn regions_in_return(&self) -> Self::ReturnRegionsIter {
    self
      .return_ty()
      .inner_regions()
      .collect::<SmallVec<[Region<'tcx>; 8]>>()
      .into_iter()
  }

  fn all_places(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Self::PlacesIter<'_> {
    self.local_decls.indices().flat_map(move |local| {
      Place::from_local(local, tcx).interior_paths(tcx, self, def_id)
    })
  }

  #[allow(unused)]
  fn write_analysis_results<A>(
    &self,
    results: &Results<'tcx, A>,
    def_id: DefId,
    tcx: TyCtxt<'tcx>,
  ) -> Result<()>
  where
    A: Analysis<'tcx>,
    A::Domain: DebugWithContext<A>,
  {
    cfg_if! {
      if #[cfg(feature = "graphviz")] {
        use rustc_graphviz as dot;
        use super::graphviz;

        let graphviz =
          graphviz::Formatter::new(self, results, graphviz::OutputStyle::AfterOnly);
        let mut buf = Vec::new();
        dot::render(&graphviz, &mut buf)?;

        let output_dir = Path::new("target");
        let fname = tcx.def_path_debug_str(def_id);
        let output_path = output_dir.join(format!("{fname}.pdf"));

        run_dot(&output_path, buf)
      } else {
        anyhow::bail!("graphviz feature is not enabled")
      }
    }
  }
}

pub fn run_dot(path: &Path, buf: Vec<u8>) -> Result<()> {
  let mut p = Command::new("dot")
    .args(["-Tpdf", "-o", &path.display().to_string()])
    .stdin(Stdio::piped())
    .spawn()?;

  p.stdin.as_mut().unwrap().write_all(&buf)?;

  let status = p.wait()?;
  ensure!(status.success(), "dot for {} failed", path.display());

  Ok(())
}

#[cfg(test)]
mod test {
  use super::BodyExt;
  use crate::test_utils;

  #[test]
  fn test_body_ext() {
    let input = r#"
    fn foobar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
      if *x > 0 {
        return x;
      }

      y
    }"#;

    test_utils::compile_body(input, |_, _, body| {
      let body = &body.body;
      assert_eq!(body.regions_in_args().count(), 2);
      assert_eq!(body.regions_in_return().count(), 1);
    });
  }
}
