//! Utilities for [`Ty`].

use rustc_data_structures::captures::Captures;
use rustc_hir::def_id::DefId;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::ty::{GenericArgKind, ParamEnv, Region, Ty, TyCtxt};
use rustc_trait_selection::infer::InferCtxtExt;

/// Extension trait for [`Ty`].
pub trait TyExt<'tcx> {
  type AllRegionsIter<'a>: Iterator<Item = Region<'tcx>>
  where
    Self: 'a;

  /// Returns an iterator over the regions appearing within a type.
  fn inner_regions(&self) -> Self::AllRegionsIter<'_>;

  /// Returns true if a type implements a given trait.
  fn does_implement_trait(
    &self,
    tcx: TyCtxt<'tcx>,
    param_env: ParamEnv<'tcx>,
    trait_def_id: DefId,
  ) -> bool;

  /// Returns true if a type implements `Copy`.
  fn is_copyable(&self, tcx: TyCtxt<'tcx>, param_env: ParamEnv<'tcx>) -> bool;
}

impl<'tcx> TyExt<'tcx> for Ty<'tcx> {
  type AllRegionsIter<'a> = impl Iterator<Item = Region<'tcx>> + Captures<'tcx> + 'a
    where Self: 'a;

  fn inner_regions(&self) -> Self::AllRegionsIter<'_> {
    self.walk().filter_map(|part| match part.unpack() {
      GenericArgKind::Lifetime(region) => Some(region),
      _ => None,
    })
  }

  fn does_implement_trait(
    &self,
    tcx: TyCtxt<'tcx>,
    param_env: ParamEnv<'tcx>,
    trait_def_id: DefId,
  ) -> bool {
    use rustc_infer::traits::EvaluationResult;

    let infcx = tcx.infer_ctxt().build();
    let ty = tcx.erase_regions(*self);
    let result = infcx.type_implements_trait(trait_def_id, [ty], param_env);
    matches!(
      result,
      EvaluationResult::EvaluatedToOk | EvaluationResult::EvaluatedToOkModuloRegions
    )
  }

  fn is_copyable(&self, tcx: TyCtxt<'tcx>, param_env: ParamEnv<'tcx>) -> bool {
    let ty = tcx.erase_regions(*self);
    ty.is_copy_modulo_regions(tcx, param_env)
  }
}

#[cfg(test)]
mod test {
  use rustc_middle::ty::ParamEnv;

  use super::TyExt;
  use crate::{test_utils, BodyExt};

  #[test]
  fn test_ty_ext() {
    let input = r#"
fn main() {
  let x = &mut 0;
  let y = 0;
}"#;

    test_utils::CompileBuilder::new(input).compile(|result| {
      let tcx = result.tcx;
      let body = result.as_body().1;
      let body = &body.body;
      let locals = body.debug_info_name_map();
      let x = &body.local_decls[locals["x"]];
      let y = &body.local_decls[locals["y"]];
      assert_eq!(x.ty.inner_regions().count(), 1);
      assert_eq!(y.ty.inner_regions().count(), 0);

      assert!(!x.ty.is_copyable(tcx, ParamEnv::empty()));
      assert!(y.ty.is_copyable(tcx, ParamEnv::empty()));
    });
  }
}
