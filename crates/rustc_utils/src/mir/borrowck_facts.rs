//! Polonius integration to extract borrowck facts from rustc.

use std::sync::atomic::{AtomicBool, Ordering};

use rustc_borrowck::consumers::{BodyWithBorrowckFacts, ConsumerOptions};
use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
  mir::{Body, BorrowCheckResult, MirPass, StatementKind, TerminatorKind},
  ty::TyCtxt,
  util::Providers,
};

use crate::{block_timer, cache::Cache, BodyExt};

/// MIR pass to remove instructions not important for Flowistry.
///
/// This pass helps reduce the number of intermediates during dataflow analysis, which
/// reduces memory usage.
pub struct SimplifyMir;
impl<'tcx> MirPass<'tcx> for SimplifyMir {
  fn run_pass(&self, _tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
    let return_blocks = body
      .all_returns()
      .filter_map(|loc| {
        let bb = &body.basic_blocks[loc.block];
        (bb.statements.len() == 0).then_some(loc.block)
      })
      .collect::<HashSet<_>>();

    for block in body.basic_blocks_mut() {
      block.statements.retain(|stmt| {
        !matches!(
          stmt.kind,
          StatementKind::StorageLive(..) | StatementKind::StorageDead(..)
        )
      });

      let terminator = block.terminator_mut();
      terminator.kind = match terminator.kind {
        TerminatorKind::FalseEdge { real_target, .. } => TerminatorKind::Goto {
          target: real_target,
        },
        TerminatorKind::FalseUnwind { real_target, .. } => TerminatorKind::Goto {
          target: real_target,
        },
        // Ensures that control dependencies can determine the independence of differnet
        // return paths
        TerminatorKind::Goto { target } if return_blocks.contains(&target) => {
          TerminatorKind::Return
        }
        _ => continue,
      }
    }
  }
}

static SIMPLIFY_MIR: AtomicBool = AtomicBool::new(false);

pub fn enable_mir_simplification() {
  SIMPLIFY_MIR.store(true, Ordering::SeqCst);
}

/// You must use this function in [`rustc_driver::Callbacks::config`] to call [`get_body_with_borrowck_facts`].
///
/// For why we need to do override mir_borrowck, see:
/// <https://github.com/rust-lang/rust/blob/485ced56b8753ec86936903f2a8c95e9be8996a1/src/test/run-make-fulldeps/obtain-borrowck/driver.rs>
pub fn override_queries(_session: &rustc_session::Session, local: &mut Providers) {
  local.mir_borrowck = mir_borrowck;
}

thread_local! {
  static MIR_BODIES: Cache<LocalDefId, BodyWithBorrowckFacts<'static>> = Cache::default();
}

fn mir_borrowck(tcx: TyCtxt<'_>, def_id: LocalDefId) -> &BorrowCheckResult<'_> {
  block_timer!(&format!(
    "get_body_with_borrowck_facts for {}",
    tcx.def_path_debug_str(def_id.to_def_id())
  ));

  let mut body_with_facts = rustc_borrowck::consumers::get_body_with_borrowck_facts(
    tcx,
    def_id,
    ConsumerOptions::PoloniusInputFacts,
  );

  if SIMPLIFY_MIR.load(Ordering::SeqCst) {
    SimplifyMir.run_pass(tcx, &mut body_with_facts.body);
  }

  // SAFETY: The reader casts the 'static lifetime to 'tcx before using it.
  let body_with_facts: BodyWithBorrowckFacts<'static> =
    unsafe { std::mem::transmute(body_with_facts) };
  MIR_BODIES.with(|cache| {
    cache.get(def_id, |_| body_with_facts);
  });

  let mut providers = Providers::default();
  rustc_borrowck::provide(&mut providers);
  let original_mir_borrowck = providers.mir_borrowck;
  original_mir_borrowck(tcx, def_id)
}

/// Gets the MIR body and [Polonius](https://github.com/rust-lang/polonius)-generated
/// [borrowck facts](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_borrowck/struct.BodyWithBorrowckFacts.html)
/// for a given [`LocalDefId`].
///
/// For this function to work, you MUST add [`override_queries`] to the
/// [`rustc_interface::Config`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Config.html)
/// inside of your [`rustc_driver::Callbacks`]. For example, see
/// [example.rs](https://github.com/willcrichton/flowistry/tree/master/crates/flowistry/examples/example.rs).
///
/// Note that as of May 2022, Polonius can be *very* slow for large functions.
/// It may take up to 30 seconds to analyze a single body with a large CFG.
#[allow(clippy::needless_lifetimes)]
pub fn get_body_with_borrowck_facts<'tcx>(
  tcx: TyCtxt<'tcx>,
  def_id: LocalDefId,
) -> &'tcx BodyWithBorrowckFacts<'tcx> {
  let _ = tcx.mir_borrowck(def_id);
  MIR_BODIES.with(|cache| {
    let body = cache.get(def_id, |_| panic!("mir_borrowck override should have stored body for item: {def_id:?}. Are you sure you registered borrowck_facts::override_queries?"));
    unsafe {
      std::mem::transmute::<
        &BodyWithBorrowckFacts<'static>,
        &'tcx BodyWithBorrowckFacts<'tcx>,
      >(body)
    }
  })
}
