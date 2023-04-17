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
