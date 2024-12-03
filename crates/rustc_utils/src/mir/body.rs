//! Utilities for [`Body`].

use std::{
  io::Write,
  path::Path,
  process::{Command, Stdio},
};

use anyhow::{ensure, Result};
use pretty::PrettyPrintMirOptions;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::{def_id::DefId, CoroutineDesugaring, CoroutineKind, HirId};
use rustc_middle::{
  mir::{
    pretty, pretty::write_mir_fn, BasicBlock, Body, Local, Location, Place, SourceInfo,
    TerminatorKind, VarDebugInfoContents,
  },
  ty::{Region, Ty, TyCtxt},
};
use smallvec::SmallVec;

use super::control_dependencies::ControlDependencies;
use crate::{PlaceExt, TyExt};

/// Extension trait for [`Body`].
pub trait BodyExt<'tcx> {
  /// Returns an iterator over the locations of [`TerminatorKind::Return`] instructions in a body.
  fn all_returns(&self) -> impl Iterator<Item = Location> + '_;

  /// Returns an iterator over all the locations in a body.
  fn all_locations(&self) -> impl Iterator<Item = Location> + '_;

  /// Returns all the locations in a [`BasicBlock`].
  fn locations_in_block(&self, block: BasicBlock) -> impl Iterator<Item = Location>;

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

  /// Returns an iterator over all projections of all local variables in the body.
  fn all_places(
    &self,
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
  ) -> impl Iterator<Item = Place<'tcx>> + '_;

  /// Returns an iterator over all the regions that appear in argument types to the body.
  fn regions_in_args(&self) -> impl Iterator<Item = Region<'tcx>> + '_;

  /// Returns an iterator over all the regions that appear in the body's return type.
  fn regions_in_return(&self) -> impl Iterator<Item = Region<'tcx>> + '_;
}

impl<'tcx> BodyExt<'tcx> for Body<'tcx> {
  fn all_returns(&self) -> impl Iterator<Item = Location> + '_ {
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

  fn all_locations(&self) -> impl Iterator<Item = Location> + '_ {
    self
      .basic_blocks
      .iter_enumerated()
      .flat_map(|(block, data)| {
        (0 ..= data.statements.len()).map(move |statement_index| Location {
          block,
          statement_index,
        })
      })
  }

  fn locations_in_block(&self, block: BasicBlock) -> impl Iterator<Item = Location> {
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
    write_mir_fn(
      tcx,
      self,
      &mut |_, _| Ok(()),
      &mut buffer,
      PrettyPrintMirOptions {
        include_extra_comments: false,
      },
    )?;
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
    if matches!(
      tcx.coroutine_kind(def_id),
      Some(CoroutineKind::Desugared(CoroutineDesugaring::Async, _))
    ) {
      Some(self.local_decls[Local::from_usize(2)].ty)
    } else {
      None
    }
  }

  fn regions_in_args(&self) -> impl Iterator<Item = Region<'tcx>> + '_ {
    self
      .args_iter()
      .flat_map(|arg_local| self.local_decls[arg_local].ty.inner_regions())
  }

  fn regions_in_return(&self) -> impl Iterator<Item = Region<'tcx>> + '_ {
    self
      .return_ty()
      .inner_regions()
      .collect::<SmallVec<[Region<'tcx>; 8]>>()
      .into_iter()
  }

  fn all_places(
    &self,
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
  ) -> impl Iterator<Item = Place<'tcx>> + '_ {
    self.local_decls.indices().flat_map(move |local| {
      Place::from_local(local, tcx).interior_paths(tcx, self, def_id)
    })
  }
}

pub fn run_dot(path: &Path, buf: &[u8]) -> Result<()> {
  let mut p = Command::new("dot")
    .args(["-Tpdf", "-o", &path.display().to_string()])
    .stdin(Stdio::piped())
    .spawn()?;

  p.stdin.as_mut().unwrap().write_all(buf)?;

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
    let input = r"
fn foobar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
  if *x > 0 {
    return x;
  }

  y
}";

    test_utils::compile_body(input, |_, _, body| {
      let body = &body.body;
      assert_eq!(body.regions_in_args().count(), 2);
      assert_eq!(body.regions_in_return().count(), 1);
    });
  }
}
