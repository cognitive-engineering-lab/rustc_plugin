use anyhow::Result;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::{def_id::DefId, GeneratorKind, HirId};
use rustc_middle::{
  mir::{pretty::write_mir_fn, *},
  ty::{Ty, TyCtxt},
};
use rustc_span::Symbol;

use super::control_dependencies::ControlDependencies;

/// Extension trait for [`Body`].
pub trait BodyExt<'tcx> {
  type AllReturnsIter<'a>: Iterator<Item = Location>
  where
    Self: 'a;

  /// Returns all the locations of [`TerminatorKind::Return`] instructions in a body.
  fn all_returns(&self) -> Self::AllReturnsIter<'_>;

  type AllLocationsIter<'a>: Iterator<Item = Location>
  where
    Self: 'a;

  /// Returns all the locations in a body.
  fn all_locations(&self) -> Self::AllLocationsIter<'_>;

  type LocationsIter: Iterator<Item = Location>;

  /// Returns all the locations in a [`BasicBlock`].
  fn locations_in_block(&self, block: BasicBlock) -> Self::LocationsIter;

  fn debug_info_name_map(&self) -> HashMap<Local, Symbol>;

  fn to_string(&self, tcx: TyCtxt<'tcx>) -> Result<String>;

  /// Returns the HirId corresponding to a MIR location.
  ///
  /// You **MUST** use the `-Zmaximize-hir-to-mir-mapping` flag for this
  /// function to work.
  fn location_to_hir_id(&self, location: Location) -> HirId;

  fn source_info_to_hir_id(&self, info: &SourceInfo) -> HirId;

  fn control_dependencies(&self) -> ControlDependencies<BasicBlock>;

  fn async_context(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<Ty<'tcx>>;
}

// https://github.com/rust-lang/rust/issues/66551#issuecomment-629815002
pub trait Captures<'a> {}
impl<'a, T> Captures<'a> for T {}

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

  fn debug_info_name_map(&self) -> HashMap<Local, Symbol> {
    self
      .var_debug_info
      .iter()
      .filter_map(|info| match info.value {
        VarDebugInfoContents::Place(place) => Some((place.local, info.name)),
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
}
