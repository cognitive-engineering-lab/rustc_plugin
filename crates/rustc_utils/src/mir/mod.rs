//! Utilities for MIR-level data structures.

pub mod adt_def;
pub mod body;
pub mod borrowck_facts;
pub mod control_dependencies;
pub mod location_or_arg;
pub mod mutability;
pub mod operand;
pub mod place;

#[allow(clippy::all, clippy::pedantic)]
pub mod places_conflict;
