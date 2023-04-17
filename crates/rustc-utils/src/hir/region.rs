//! Utilities for [`Region`].

use rustc_middle::ty::{Region, RegionVid};

/// Extension trait for [`Region`].
pub trait RegionExt {
  /// Assume that the region is a [`RegionVid`], getting the variable if so
  /// and panicing otherwise.
  fn to_region_vid(self) -> RegionVid;
}

impl<'tcx> RegionExt for Region<'tcx> {
  fn to_region_vid(self) -> RegionVid {
    self
      .as_var()
      .unwrap_or_else(|| panic!("region is not an ReVar: {self:?}"))
  }
}
