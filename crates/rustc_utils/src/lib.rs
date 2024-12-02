//! `rustc_utils` provides a wide variety of utilities for working with the Rust compiler.
//! We developed these functions in the course of building various research projects with
//! rustc.
//!
//! Most of the functionality is organized into extension traits implemented for types
//! in the compiler, such as one for MIR control-flow graphs ([`BodyExt`]) or one for
//! text ranges ([`SpanExt`]).
//!
//! This crate is pinned to a specific nightly version of the Rust compiler.
//! See the [`rustc_plugin` README](https://github.com/cognitive-engineering-lab/rustc_plugin)
//! for details on how to add `rustc_utils` as a dependency.

#![feature(
  rustc_private,
  negative_impls,        // for !Send
  min_specialization,    // for rustc_index::newtype_index 
  type_alias_impl_trait, // for iterators in traits
  box_patterns,          // for ergonomics
  let_chains,            // for places_conflict module
  exact_size_is_empty,   // for graphviz module
  impl_trait_in_assoc_type,
  doc_auto_cfg,          // for feature gates in documentation
)]
#![allow(clippy::len_zero, clippy::len_without_is_empty)]

extern crate either;
extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_graphviz;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_macros;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_mir_transform;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_type_ir;
extern crate smallvec;

pub mod cache;
pub mod hir;
pub mod mir;
pub mod source_map;
#[cfg(feature = "test")]
pub mod test_utils;
pub mod timer;

pub use crate::{
  hir::ty::TyExt,
  mir::{
    adt_def::AdtDefExt, body::BodyExt, mutability::MutabilityExt, operand::OperandExt,
    place::PlaceExt,
  },
  source_map::span::{SpanDataExt, SpanExt},
};

/// Utility for hashset literals. Same as maplit::hashset but works with FxHasher.
#[macro_export]
macro_rules! hashset {
  (@single $($x:tt)*) => (());
  (@count $($rest:expr),*) => (<[()]>::len(&[$(hashset!(@single $rest)),*]));

  ($($key:expr,)+) => { hashset!($($key),+) };
  ($($key:expr),*) => {
      {
          let _cap = hashset!(@count $($key),*);
          let mut _set = ::rustc_data_structures::fx::FxHashSet::default();
          let _ = _set.try_reserve(_cap);
          $(
              let _ = _set.insert($key);
          )*
          _set
      }
  };
}
