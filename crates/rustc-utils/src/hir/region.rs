use rustc_middle::ty::{self, Region, RegionVid};

pub trait RegionExt {
  fn to_region_vid(self) -> RegionVid;
}

impl<'tcx> RegionExt for Region<'tcx> {
  // XXX: when our pinned toolchain is upgraded we can
  // use `Region::as_var` instead to make this simpler.
  fn to_region_vid(self) -> RegionVid {
    if let ty::ReVar(vid) = self.kind() {
      vid
    } else {
      unreachable!("region is not an ReVar{:?}", self)
    }
  }
}
