use either::Either;
use rustc_middle::mir::{Body, Local, Location, Place};

use crate::PlaceExt;

/// Used to represent dependencies of places.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationOrArg {
  Location(Location),
  Arg(Local),
}

impl LocationOrArg {
  pub fn from_place<'tcx>(place: Place<'tcx>, body: &Body<'tcx>) -> Option<Self> {
    place
      .is_arg(body)
      .then_some(LocationOrArg::Arg(place.local))
  }

  pub fn to_string(self, body: &Body<'_>) -> String {
    match self {
      LocationOrArg::Arg(local) => format!("{local:?}"),
      LocationOrArg::Location(location) => match body.stmt_at(location) {
        Either::Left(stmt) => format!("{:?}", stmt.kind),
        Either::Right(terminator) => format!("{:?}", terminator.kind),
      },
    }
  }
}

impl From<Location> for LocationOrArg {
  fn from(location: Location) -> Self {
    LocationOrArg::Location(location)
  }
}

impl From<Local> for LocationOrArg {
  fn from(local: Local) -> Self {
    LocationOrArg::Arg(local)
  }
}

#[cfg(feature = "indexical")]
pub mod index {
  use indexical::{bitset::rustc::IndexSet, define_index_type, IndexedDomain, ToIndex};

  use super::*;

  define_index_type! {
    pub struct LocationOrArgIndex for LocationOrArg = u32;
  }

  pub type LocationOrArgSet = IndexSet<LocationOrArg>;
  pub type LocationOrArgDomain = IndexedDomain<LocationOrArg>;

  pub struct CustomMarker;

  impl ToIndex<LocationOrArg, CustomMarker> for Location {
    fn to_index(self, domain: &IndexedDomain<LocationOrArg>) -> LocationOrArgIndex {
      LocationOrArg::Location(self).to_index(domain)
    }
  }

  impl ToIndex<LocationOrArg, CustomMarker> for Local {
    fn to_index(self, domain: &IndexedDomain<LocationOrArg>) -> LocationOrArgIndex {
      LocationOrArg::Arg(self).to_index(domain)
    }
  }

  impl rustc_index::Idx for LocationOrArgIndex {
    fn new(idx: usize) -> Self {
      LocationOrArgIndex::new(idx)
    }

    fn index(self) -> usize {
      self.index()
    }
  }
}
