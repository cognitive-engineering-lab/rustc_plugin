pub mod body;
pub mod borrowck_facts;
pub mod control_dependencies;
pub mod mutability;
pub mod operand;
pub mod place;

#[cfg(feature = "graphviz")]
#[allow(clippy::all, clippy::pedantic)]
pub mod graphviz;
#[allow(clippy::all, clippy::pedantic)]
pub mod places_conflict;
