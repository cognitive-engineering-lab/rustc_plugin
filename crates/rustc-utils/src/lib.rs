#![feature(
  rustc_private,
  negative_impls,        // for !Send
  min_specialization,    // for rustc_index::newtype_index 
  type_alias_impl_trait, // for iterators in traits
  lazy_cell,             // for global constants w/ heap allocation
  box_patterns
)]
#![allow(clippy::len_zero)]

extern crate either;
extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_graphviz;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
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
extern crate smallvec;

pub mod cache;
pub mod mir;
pub mod source_map;
#[cfg(feature = "test")]
pub mod test_utils;
pub mod timer;

pub use crate::{
  mir::{body::BodyExt, mutability::MutabilityExt, operand::OperandExt, place::PlaceExt},
  source_map::span::{SpanDataExt, SpanExt},
};
