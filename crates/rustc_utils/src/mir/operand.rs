//! Utilities for [`Operand`].

use rustc_middle::mir::{Operand, Place};

/// Extension trait for [`Operand`].
pub trait OperandExt<'tcx> {
  /// Extracts the [`Place`] inside an [`Operand`] if it exists.
  fn as_place(&self) -> Option<Place<'tcx>>;
}

impl<'tcx> OperandExt<'tcx> for Operand<'tcx> {
  fn as_place(&self) -> Option<Place<'tcx>> {
    match self {
      Operand::Copy(place) | Operand::Move(place) => Some(*place),
      Operand::Constant(_) => None,
    }
  }
}
