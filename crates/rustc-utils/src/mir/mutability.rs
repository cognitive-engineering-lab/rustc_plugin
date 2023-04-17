use rustc_middle::mir::Mutability;

pub trait MutabilityExt {
  /// Returns true if `self` is eqully or more permissive than `other`,
  /// i.e. where `Not` is more permissive than `Mut`.
  ///
  /// This corresponds to the relation $\omega_1 \lesssim \omega_2$ in the Flowistry paper.
  #[allow(clippy::wrong_self_convention)]
  fn is_permissive_as(self, other: Self) -> bool;
}

impl MutabilityExt for Mutability {
  fn is_permissive_as(self, other: Self) -> bool {
    !matches!((self, other), (Mutability::Mut, Mutability::Not))
  }
}

#[test]
fn test_mutability() {
  use Mutability::*;
  let truth_table = [
    (Not, Not, true),
    (Not, Mut, true),
    (Mut, Not, false),
    (Mut, Mut, true),
  ];
  for (l, r, v) in truth_table {
    assert_eq!(l.is_permissive_as(r), v);
  }
}
