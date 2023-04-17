use rustc_middle::mir::Mutability;

pub trait MutabilityExt {
  /// Returns true if `self` is eqully or more permissive than `other`,
  /// i.e. where `Not` is more permissive than `Mut`.
  ///
  /// This corresponds to the operation $\omega_1 \lesssim \omega_2$ in the Flowistry paper.
  fn more_permissive_than(self, other: Self) -> bool;
}

impl MutabilityExt for Mutability {
  fn more_permissive_than(self, other: Self) -> bool {
    !matches!((self, other), (Mutability::Not, Mutability::Mut))
  }
}
