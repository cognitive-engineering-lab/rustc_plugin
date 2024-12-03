//! Utilities for [`AdtDef`].

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{AdtDef, FieldDef, TyCtxt};

/// Extension trait for [`AdtDef`].
pub trait AdtDefExt<'tcx> {
  /// Returns an iterator over all the fields of the ADT that are visible
  /// from `module`.
  fn all_visible_fields(
    self,
    module: DefId,
    tcx: TyCtxt<'tcx>,
  ) -> impl Iterator<Item = &'tcx FieldDef>;
}

impl<'tcx> AdtDefExt<'tcx> for AdtDef<'tcx> {
  fn all_visible_fields(
    self,
    module: DefId,
    tcx: TyCtxt<'tcx>,
  ) -> impl Iterator<Item = &'tcx FieldDef> {
    self
      .all_fields()
      .filter(move |field| field.vis.is_accessible_from(module, tcx))
  }
}
