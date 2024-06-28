//! Utilities for [`Place`].

use std::{borrow::Cow, collections::VecDeque, ops::ControlFlow};

use log::{trace, warn};
use rustc_data_structures::fx::{FxHashMap as HashMap, FxHashSet as HashSet};
use rustc_hir::def_id::DefId;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::{
  mir::{
    visit::{PlaceContext, Visitor},
    Body, HasLocalDecls, Local, Location, MirPass, Mutability, Place, PlaceElem,
    PlaceRef, ProjectionElem, StatementKind, TerminatorKind, VarDebugInfo,
    VarDebugInfoContents, RETURN_PLACE,
  },
  traits::ObligationCause,
  ty::{self, AdtKind, Region, RegionKind, RegionVid, Ty, TyCtxt, TyKind, TypeVisitor},
};
use rustc_target::abi::{FieldIdx, VariantIdx};
use rustc_trait_selection::traits::NormalizeExt;

use crate::{AdtDefExt, BodyExt, SpanExt};

/// A MIR [`Visitor`] which collects all [`Place`]s that appear in the visited object.
#[derive(Default)]
pub struct PlaceCollector<'tcx>(pub Vec<Place<'tcx>>);

impl<'tcx> Visitor<'tcx> for PlaceCollector<'tcx> {
  fn visit_place(
    &mut self,
    place: &Place<'tcx>,
    _context: PlaceContext,
    _location: Location,
  ) {
    self.0.push(*place);
  }
}

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
        bb.statements.is_empty().then_some(loc.block)
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

/// Extension trait for [`Place`].
pub trait PlaceExt<'tcx> {
  /// Creates a new [`Place`].
  fn make(local: Local, projection: &[PlaceElem<'tcx>], tcx: TyCtxt<'tcx>) -> Self;

  /// Converts a [`PlaceRef`] into an owned [`Place`].
  fn from_ref(place: PlaceRef<'tcx>, tcx: TyCtxt<'tcx>) -> Self;

  /// Creates a new [`Place`] with an empty projection.
  fn from_local(local: Local, tcx: TyCtxt<'tcx>) -> Self;

  /// Returns true if `self` is a projection of an argument local.
  fn is_arg(&self, body: &Body<'tcx>) -> bool;

  /// Returns true if `self` could not be resolved further to another place.
  ///
  /// This is true of places with no dereferences in the projection, or of dereferences
  /// of arguments.
  fn is_direct(&self, body: &Body<'tcx>) -> bool;

  type RefsInProjectionIter<'a>: Iterator<
    Item = (PlaceRef<'tcx>, &'tcx [PlaceElem<'tcx>]),
  >
  where
    Self: 'a;

  /// Returns an iterator over all prefixes of `self`'s projection that are references,
  ///  along with the suffix of the remaining projection.
  fn refs_in_projection(&self) -> Self::RefsInProjectionIter<'_>;

  /// Returns all possible projections of `self` that are references.
  ///
  /// The output data structure groups the resultant places based on the region of the references.
  fn interior_pointers(
    &self,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    def_id: DefId,
  ) -> HashMap<RegionVid, Vec<(Place<'tcx>, Mutability)>>;

  /// Returns all possible projections of `self` that do not go through a reference,
  /// i.e. the set of fields directly in the structure referred by `self`.
  fn interior_places(
    &self,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    def_id: DefId,
  ) -> Vec<Place<'tcx>>;

  /// Returns all possible projections of `self`.
  fn interior_paths(
    &self,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    def_id: DefId,
  ) -> Vec<Place<'tcx>>;

  /// Returns a pretty representation of a place that uses debug info when available.
  fn to_string(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) -> Option<String>;

  /// Erases/normalizes information in a place to ensure stable comparisons between places.
  ///
  /// Consider a place `_1: &'1 <T as SomeTrait>::Foo[2]`.
  ///   We might encounter this type with a different region, e.g. `&'2`.
  ///   We might encounter this type with a more specific type for the associated type, e.g. `&'1 [i32][0]`.
  /// To account for this variation, we normalize associated types,
  ///   erase regions, and normalize projections.
  fn normalize(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Place<'tcx>;

  /// Returns true if this place's base [`Local`] corresponds to code that is visible in the source.
  fn is_source_visible(&self, tcx: TyCtxt, body: &Body) -> bool;
}

impl<'tcx> PlaceExt<'tcx> for Place<'tcx> {
  fn make(local: Local, projection: &[PlaceElem<'tcx>], tcx: TyCtxt<'tcx>) -> Self {
    Place {
      local,
      projection: tcx.mk_place_elems(projection),
    }
  }

  fn from_ref(place: PlaceRef<'tcx>, tcx: TyCtxt<'tcx>) -> Self {
    Self::make(place.local, place.projection, tcx)
  }

  fn from_local(local: Local, tcx: TyCtxt<'tcx>) -> Self {
    Place::make(local, &[], tcx)
  }

  fn is_arg(&self, body: &Body<'tcx>) -> bool {
    let i = self.local.as_usize();
    i > 0 && i - 1 < body.arg_count
  }

  fn is_direct(&self, body: &Body<'tcx>) -> bool {
    !self.is_indirect() || self.is_arg(body)
  }

  type RefsInProjectionIter<'a> = impl Iterator<Item = (PlaceRef<'tcx>, &'tcx [PlaceElem<'tcx>])> + 'a where Self: 'a;
  fn refs_in_projection(&self) -> Self::RefsInProjectionIter<'_> {
    self
      .projection
      .iter()
      .enumerate()
      .filter_map(|(i, elem)| match elem {
        ProjectionElem::Deref => {
          let ptr = PlaceRef {
            local: self.local,
            projection: &self.projection[.. i],
          };
          let after = &self.projection[i + 1 ..];
          Some((ptr, after))
        }
        _ => None,
      })
  }

  fn interior_pointers(
    &self,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    def_id: DefId,
  ) -> HashMap<RegionVid, Vec<(Place<'tcx>, Mutability)>> {
    let ty = self.ty(body.local_decls(), tcx).ty;
    let mut region_collector = CollectRegions {
      tcx,
      def_id,
      local: self.local,
      place_stack: self.projection.to_vec(),
      ty_stack: Vec::new(),
      regions: HashMap::default(),
      places: None,
      types: None,
      stop_at: if
      /*shallow*/
      false {
        StoppingCondition::AfterRefs
      } else {
        StoppingCondition::None
      },
    };
    region_collector.visit_ty(ty);
    region_collector.regions
  }

  fn interior_places(
    &self,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    def_id: DefId,
  ) -> Vec<Place<'tcx>> {
    let ty = self.ty(body.local_decls(), tcx).ty;
    let mut region_collector = CollectRegions {
      tcx,
      def_id,
      local: self.local,
      place_stack: self.projection.to_vec(),
      ty_stack: Vec::new(),
      regions: HashMap::default(),
      places: Some(HashSet::default()),
      types: None,
      stop_at: StoppingCondition::BeforeRefs,
    };
    region_collector.visit_ty(ty);
    region_collector.places.unwrap().into_iter().collect()
  }

  fn interior_paths(
    &self,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    def_id: DefId,
  ) -> Vec<Place<'tcx>> {
    let ty = self.ty(body.local_decls(), tcx).ty;
    let mut region_collector = CollectRegions {
      tcx,
      def_id,
      local: self.local,
      place_stack: self.projection.to_vec(),
      ty_stack: Vec::new(),
      regions: HashMap::default(),
      places: Some(HashSet::default()),
      types: None,
      stop_at: StoppingCondition::None,
    };
    region_collector.visit_ty(ty);
    region_collector.places.unwrap().into_iter().collect()
  }

  fn to_string(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) -> Option<String> {
    // Get the local's debug name from the Body's VarDebugInfo
    let local_name = if self.local == RETURN_PLACE {
      Cow::Borrowed("RETURN")
    } else {
      let get_local_name = |info: &VarDebugInfo<'tcx>| match info.value {
        VarDebugInfoContents::Place(place) if place.local == self.local => info
          .source_info
          .span
          .as_local(body.span)
          .map(|_| info.name.to_string()),
        _ => None,
      };
      let local_name = body.var_debug_info.iter().find_map(get_local_name)?;
      Cow::Owned(local_name)
    };

    #[derive(Copy, Clone)]
    enum ElemPosition {
      Prefix,
      Suffix,
    }

    // Turn each PlaceElem into a prefix (e.g. * for deref) or a suffix
    // (e.g. .field for projection).
    let elem_to_string = |(index, (place, elem)): (
      usize,
      (PlaceRef<'tcx>, PlaceElem<'tcx>),
    )|
     -> (ElemPosition, Cow<'static, str>) {
      match elem {
        ProjectionElem::Deref => (ElemPosition::Prefix, "*".into()),

        ProjectionElem::Field(field, _) => {
          let ty = place.ty(&body.local_decls, tcx).ty;

          let field_name = match ty.kind() {
            TyKind::Adt(def, _substs) => {
              let fields = match def.adt_kind() {
                AdtKind::Struct => &def.non_enum_variant().fields,
                AdtKind::Enum => {
                  let Some(PlaceElem::Downcast(_, variant_idx)) =
                    self.projection.get(index - 1)
                  else {
                    unimplemented!()
                  };
                  &def.variant(*variant_idx).fields
                }
                kind => unimplemented!("{kind:?}"),
              };

              fields[field].ident(tcx).to_string()
            }

            TyKind::Tuple(_) => field.as_usize().to_string(),

            TyKind::Closure(def_id, _substs) => match def_id.as_local() {
              Some(local_def_id) => {
                let captures = tcx.closure_captures(local_def_id);
                captures[field.as_usize()].var_ident.to_string()
              }
              None => field.as_usize().to_string(),
            },

            kind => unimplemented!("{kind:?}"),
          };

          (ElemPosition::Suffix, format!(".{field_name}").into())
        }
        ProjectionElem::Downcast(sym, _) => {
          let variant = sym.map(|s| s.to_string()).unwrap_or_else(|| "??".into());
          (ElemPosition::Suffix, format!("@{variant}",).into())
        }

        ProjectionElem::Index(_) => (ElemPosition::Suffix, "[_]".into()),
        kind => unimplemented!("{kind:?}"),
      }
    };

    let (positions, contents): (Vec<_>, Vec<_>) = self
      .iter_projections()
      .enumerate()
      .map(elem_to_string)
      .unzip();

    // Combine the prefixes and suffixes into a corresponding sequence
    let mut parts = VecDeque::from([local_name]);
    for (i, string) in contents.into_iter().enumerate() {
      match positions[i] {
        ElemPosition::Prefix => {
          parts.push_front(string);
          if matches!(positions.get(i + 1), Some(ElemPosition::Suffix)) {
            parts.push_front(Cow::Borrowed("("));
            parts.push_back(Cow::Borrowed(")"));
          }
        }
        ElemPosition::Suffix => parts.push_back(string),
      }
    }

    let full = parts.make_contiguous().join("");
    Some(full)
  }

  fn normalize(&self, tcx: TyCtxt<'tcx>, def_id: DefId) -> Place<'tcx> {
    let param_env = tcx.param_env(def_id);
    let place = tcx.erase_regions(*self);
    let infcx = tcx.infer_ctxt().build();
    let place = infcx
      .at(&ObligationCause::dummy(), param_env)
      .normalize(place)
      .value;

    let projection = place
      .projection
      .into_iter()
      .filter_map(|elem| match elem {
        // Map all indexes [i] to [0] since they should be considered equal
        ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. } => {
          Some(ProjectionElem::Index(Local::from_usize(0)))
        }
        // Ignore subslices, they should be treated the same as the
        // full slice
        ProjectionElem::Subslice { .. } => None,
        _ => Some(elem),
      })
      .collect::<Vec<_>>();

    Place::make(place.local, &projection, tcx)
  }

  fn is_source_visible(&self, _tcx: TyCtxt, body: &Body) -> bool {
    let local = self.local;
    let local_info = &body.local_decls[local];
    let is_loc = local_info.is_user_variable();
    let from_desugaring = local_info.from_compiler_desugaring();
    let from_expansion = local_info.source_info.span.from_expansion();

    // The assumption is that for a place to be source visible it needs to:
    // 1. Be a local declaration.
    // 2. Not be from a compiler desugaring.
    // 3. Not be from a macro expansion (basically also a desugaring).
    is_loc && !from_desugaring && !from_expansion
  }
}

#[derive(Copy, Clone)]
enum StoppingCondition {
  None,
  BeforeRefs,
  AfterRefs,
}

struct CollectRegions<'tcx> {
  tcx: TyCtxt<'tcx>,
  def_id: DefId,
  local: Local,
  place_stack: Vec<PlaceElem<'tcx>>,
  ty_stack: Vec<Ty<'tcx>>,
  places: Option<HashSet<Place<'tcx>>>,
  types: Option<HashSet<Ty<'tcx>>>,
  regions: HashMap<RegionVid, Vec<(Place<'tcx>, Mutability)>>,
  stop_at: StoppingCondition,
}

/// Used to describe aliases of owned and raw pointers.
pub const UNKNOWN_REGION: RegionVid = RegionVid::MAX;

impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for CollectRegions<'tcx> {
  type Result = ControlFlow<()>;

  fn visit_ty(&mut self, ty: Ty<'tcx>) -> Self::Result {
    let tcx = self.tcx;
    if self.ty_stack.iter().any(|visited_ty| ty == *visited_ty) {
      return ControlFlow::Continue(());
    }

    trace!(
      "exploring {:?} with {ty:?}",
      Place::make(self.local, &self.place_stack, tcx)
    );

    self.ty_stack.push(ty);

    match ty.kind() {
      _ if ty.is_box() => {
        self.visit_region(Region::new_var(tcx, UNKNOWN_REGION));
        self.place_stack.push(ProjectionElem::Deref);
        self.visit_ty(ty.boxed_ty());
        self.place_stack.pop();
      }

      TyKind::Tuple(fields) => {
        for (i, field) in fields.iter().enumerate() {
          self
            .place_stack
            .push(ProjectionElem::Field(FieldIdx::from_usize(i), field));
          self.visit_ty(field);
          self.place_stack.pop();
        }
      }

      TyKind::Adt(adt_def, subst) => match adt_def.adt_kind() {
        ty::AdtKind::Struct => {
          for (i, field) in adt_def.all_visible_fields(self.def_id, tcx).enumerate() {
            let ty = field.ty(tcx, subst);
            self
              .place_stack
              .push(ProjectionElem::Field(FieldIdx::from_usize(i), ty));
            self.visit_ty(ty);
            self.place_stack.pop();
          }
        }
        ty::AdtKind::Union => {
          // unsafe, so ignore
        }
        ty::AdtKind::Enum => {
          for (i, variant) in adt_def.variants().iter().enumerate() {
            let variant_index = VariantIdx::from_usize(i);
            let cast = PlaceElem::Downcast(
              Some(adt_def.variant(variant_index).ident(tcx).name),
              variant_index,
            );
            self.place_stack.push(cast);
            for (j, field) in variant.fields.iter().enumerate() {
              let ty = field.ty(tcx, subst);
              let field = ProjectionElem::Field(FieldIdx::from_usize(j), ty);
              self.place_stack.push(field);
              self.visit_ty(ty);
              self.place_stack.pop();
            }
            self.place_stack.pop();
          }
        }
      },

      TyKind::Array(elem_ty, _) | TyKind::Slice(elem_ty) => {
        self
          .place_stack
          .push(ProjectionElem::Index(Local::from_usize(0)));
        self.visit_ty(*elem_ty);
        self.place_stack.pop();
      }

      TyKind::Ref(region, elem_ty, _) => match self.stop_at {
        StoppingCondition::None => {
          self.visit_region(*region);
          self.place_stack.push(ProjectionElem::Deref);
          self.visit_ty(*elem_ty);
          self.place_stack.pop();
        }
        StoppingCondition::AfterRefs => {
          self.visit_region(*region);
        }
        StoppingCondition::BeforeRefs => {}
      },

      TyKind::Closure(_, substs) | TyKind::Coroutine(_, substs) => {
        self.visit_ty(substs.as_closure().tupled_upvars_ty());
      }

      TyKind::RawPtr(ty, _) => {
        self.visit_region(Region::new_var(tcx, UNKNOWN_REGION));
        self.place_stack.push(ProjectionElem::Deref);
        self.visit_ty(*ty);
        self.place_stack.pop();
      }

      TyKind::FnDef(..)
      | TyKind::FnPtr(..)
      | TyKind::Foreign(..)
      | TyKind::Dynamic(..)
      | TyKind::Param(..)
      | TyKind::Never => {}

      _ if ty.is_primitive_ty() => {}

      _ => warn!("unimplemented {ty:?} ({:?})", ty.kind()),
    };

    // let inherent_impls = tcx.inherent_impls(self.def_id);
    // let traits = tcx.infer_ctxt().enter(|infcx| {
    //   let param_env = tcx.param_env(self.def_id);
    //   self
    //     .tcx
    //     .all_traits()
    //     .filter(|trait_def_id| {
    //       let result = infcx.type_implements_trait(*trait_def_id, ty, params, param_env);
    //       matches!(
    //         result,
    //         EvaluationResult::EvaluatedToOk
    //           | EvaluationResult::EvaluatedToOkModuloRegions
    //       )
    //     })
    //     .collect::<Vec<_>>()
    // });

    // let fns = inherent_impls.iter().chain(&traits).flat_map(|def_id| {
    //   let items = tcx.associated_items(def_id).in_definition_order();
    //   items.filter_map(|item| match item.kind {
    //     AssocKind::Fn => Some(tcx.fn_sig(item.def_id)),
    //     _ => None,
    //   })
    // });

    // // for f in fns {
    // //   f.
    // // }

    if let Some(places) = self.places.as_mut() {
      places.insert(Place::make(self.local, &self.place_stack, tcx));
    }

    if let Some(types) = self.types.as_mut() {
      types.insert(ty);
    }

    self.ty_stack.pop();
    ControlFlow::Continue(())
  }

  fn visit_region(&mut self, region: ty::Region<'tcx>) -> Self::Result {
    trace!("visiting region {region:?}");
    let region = match region.kind() {
      RegionKind::ReVar(region) => region,
      RegionKind::ReStatic => RegionVid::from_usize(0),
      RegionKind::ReErased | RegionKind::ReLateParam(_) => {
        return ControlFlow::Continue(());
      }
      _ => unreachable!("{:?}: {:?}", self.ty_stack.first().unwrap(), region),
    };

    let mutability = if self
      .ty_stack
      .iter()
      .any(|ty| ty.is_ref() && ty.ref_mutability().unwrap() == Mutability::Not)
    {
      Mutability::Not
    } else {
      Mutability::Mut
    };

    let place = Place::make(self.local, &self.place_stack, self.tcx);

    self
      .regions
      .entry(region)
      .or_default()
      .push((place, mutability));

    // for initialization setup of Aliases::build
    if let Some(places) = self.places.as_mut() {
      places.insert(self.tcx.mk_place_deref(place));
    }

    ControlFlow::Continue(())
  }
}

#[cfg(test)]
mod test {
  use rustc_borrowck::consumers::BodyWithBorrowckFacts;
  use rustc_hir::BodyId;
  use rustc_middle::{
    mir::{Place, PlaceElem},
    ty::TyCtxt,
  };

  use super::{BodyExt, PlaceExt};
  use crate::test_utils::{self, compare_sets, Placer};

  #[test]
  fn test_place_arg_direct() {
    let input = r#"
fn foobar(x: &i32) {
  let y = 1;
  let z = &y;
}
"#;
    test_utils::compile_body(input, |tcx, _, body_with_facts| {
      let body = &body_with_facts.body;
      let name_map = body.debug_info_name_map();
      let x = Place::from_local(name_map["x"], tcx);
      assert!(x.is_arg(body));
      assert!(x.is_direct(body));
      assert!(Place::make(x.local, &[PlaceElem::Deref], tcx).is_direct(body));

      let y = Place::from_local(name_map["y"], tcx);
      assert!(!y.is_arg(body));
      assert!(y.is_direct(body));

      let z = Place::from_local(name_map["z"], tcx);
      assert!(!z.is_arg(body));
      assert!(z.is_direct(body));
      assert!(!Place::make(z.local, &[PlaceElem::Deref], tcx).is_direct(body));
    });
  }

  #[test]
  fn test_place_to_string() {
    let input = r#"
struct Point { x: usize, y: usize }
fn main() {
  let x = (0, 0);
  let y = Some(1);
  let z = &[Some((0, 1))];    
  let w = (&y,);
  let p = &Point { x: 0, y: 0 };
}
    "#;
    test_utils::compile_body(input, |tcx, _, body_with_facts| {
      let body = &body_with_facts.body;
      let p = Placer::new(tcx, body);

      let x = p.local("x").mk();
      let x_1 = p.local("x").field(1).mk();
      let y_some_0 = p.local("y").downcast(1).field(0).mk();
      let z_deref_some_0_1 = p
        .local("z")
        .deref()
        .index(0)
        .downcast(1)
        .field(0)
        .field(1)
        .mk();
      let w_0_deref = p.local("w").field(0).deref().mk();
      let w_0_deref_some = p.local("w").field(0).deref().downcast(1).mk();
      let p_deref_x = p.local("p").deref().field(0).mk();

      let tests = [
        (x, "x"),
        (x_1, "x.1"),
        (y_some_0, "y@Some.0"),
        (z_deref_some_0_1, "(*z)[_]@Some.0.1"),
        (w_0_deref, "*w.0"),
        (w_0_deref_some, "(*w.0)@Some"),
        (p_deref_x, "(*p).x"),
      ];

      for (place, expected) in tests {
        assert_eq!(place.to_string(tcx, body).unwrap(), expected);
      }
    });
  }

  #[test]
  fn test_place_visitors() {
    let input = r#"
fn main() {
  let x = 0;
  let y = (0, &x);
}
    "#;
    fn callback<'tcx>(
      tcx: TyCtxt<'tcx>,
      body_id: BodyId,
      body_with_facts: &BodyWithBorrowckFacts<'tcx>,
    ) {
      let body = &body_with_facts.body;
      let def_id = tcx.hir().body_owner_def_id(body_id).to_def_id();
      let p = Placer::new(tcx, body);

      let y = p.local("y").mk();
      let y0 = p.local("y").field(0).mk();
      let y1 = p.local("y").field(1).mk();
      let y1_deref = p.local("y").field(1).deref().mk();

      compare_sets(y.interior_paths(tcx, body, def_id), [y, y0, y1, y1_deref]);

      compare_sets(y.interior_places(tcx, body, def_id), [y, y0, y1]);

      compare_sets(
        y.interior_pointers(tcx, body, def_id)
          .into_values()
          .flat_map(|vs| vs.into_iter().map(|(p, _)| p)),
        [y1],
      );
    }
    test_utils::compile_body(input, callback);
  }
}
