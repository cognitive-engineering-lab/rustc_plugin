use log::trace;
use rustc_hir::{intravisit::Visitor, BodyId};
use rustc_middle::{hir::nested_filter::OnlyBodies, ty::TyCtxt};
use rustc_span::Span;

use crate::{block_timer, SpanExt};

struct BodyFinder<'tcx> {
  tcx: TyCtxt<'tcx>,
  bodies: Vec<(Span, BodyId)>,
}

impl<'tcx> Visitor<'tcx> for BodyFinder<'tcx> {
  type NestedFilter = OnlyBodies;

  fn visit_nested_body(&mut self, id: BodyId) {
    let tcx = self.tcx;
    let hir = tcx.hir();

    // const/static items are considered to have bodies, so we want to exclude
    // them from our search for functions
    if !tcx
      .hir_body_owner_kind(tcx.hir_body_owner_def_id(id))
      .is_fn_or_closure()
    {
      return;
    }

    let body = tcx.hir_body(id);
    self.visit_body(body);

    let span = hir.span_with_body(tcx.hir_body_owner(id));
    trace!(
      "Searching body for {:?} with span {span:?} (local {:?})",
      self
        .tcx
        .def_path_debug_str(tcx.hir_body_owner_def_id(id).to_def_id()),
      span.as_local(body.value.span)
    );

    if !span.from_expansion() {
      self.bodies.push((span, id));
    }
  }
}

/// Finds all bodies in the current crate
pub fn find_bodies(tcx: TyCtxt) -> Vec<(Span, BodyId)> {
  block_timer!("find_bodies");
  let mut finder = BodyFinder {
    tcx,
    bodies: Vec::new(),
  };
  tcx.hir_visit_all_item_likes_in_crate(&mut finder);
  finder.bodies
}

/// Finds all the bodies that enclose the given span, from innermost to outermost
pub fn find_enclosing_bodies(tcx: TyCtxt, sp: Span) -> impl Iterator<Item = BodyId> {
  let mut bodies = find_bodies(tcx);
  bodies.retain(|(other, _)| other.contains(sp));
  bodies.sort_by_key(|(span, _)| span.size());
  bodies.into_iter().map(|(_, id)| id)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_utils::{self, CompileResult};

  #[test]
  fn test_find_bodies() {
    let input = r"
// Ignore constants
const C: usize = 0;

fn a() {
  // Catch nested bodies
  fn b() {}
}

fn c() {}

macro_rules! m {
  () => { fn d() {} }
}

// Ignore macro-generated bodies
m!{}
";
    test_utils::CompileBuilder::new(input).compile(|CompileResult { tcx }| {
      assert_eq!(find_bodies(tcx).len(), 3);
    });
  }
}
