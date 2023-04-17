use rustc_data_structures::captures::Captures;
use rustc_hir::def_id::DefId;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::ty::{subst::GenericArgKind, ParamEnv, Region, Ty, TyCtxt};
use rustc_trait_selection::infer::InferCtxtExt;

/// Extension trait for [`ty::Ty`]
pub trait TyExt<'tcx> {
  type AllRegionsIter<'a>: Iterator<Item = Region<'tcx>>
  where
    Self: 'a;

  fn inner_regions(&self) -> Self::AllRegionsIter<'_>;

  fn does_implement_trait(
    &self,
    tcx: TyCtxt<'tcx>,
    param_env: ParamEnv<'tcx>,
    trait_def_id: DefId,
  ) -> bool;

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
