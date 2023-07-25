//! Data structures for memoizing computations.
//! 
//! Contruct new caches using [`Default::default`], then construct/retrieve
//! elements with `get`.
//! 
//! In terms of choice, 
//! - [`CopyCache`] should be used for expensive computations that create cheap
//!   (i.e. small) values.
//! - [`Cache`] should be used for expensive computations that create expensive
//!   (i.e. large) values.
//! - [`RecursionBreakingCache`] is the same as [`Cache`] except that it should
//!   be used if the construction function for your values is going to call
//!   `get` on the same cache. It detects if such a call is made with the same
//!   key, which causes infinite recursion when [`Cache`] is used.
//! 
//! When in doubt the safest version is [`RecursionBreakingCache`] and if you
//! don't anticipate recursion to occur you can always call `unwrap()` on it's
//! result.

use std::{cell::RefCell, hash::Hash, mem, pin::Pin};

use rustc_data_structures::fx::FxHashMap as HashMap;

/// Cache for non-copyable types.
pub struct Cache<In, Out>(RefCell<HashMap<In, Pin<Box<Out>>>>);

impl<In, Out> Cache<In, Out>
where
  In: Hash + Eq + Clone,
{
  /// Size of the cache
  pub fn len(&self) -> usize {
    self.0.borrow().len()
  }
  /// Returns the cached value for the given key, or runs `compute` if
  /// the value is not in cache.
  pub fn get<'a>(&'a self, key: In, compute: impl FnOnce(In) -> Out) -> &'a Out {
    if !self.0.borrow().contains_key(&key) {
      let out = Box::pin(compute(key.clone()));
      self.0.borrow_mut().insert(key.clone(), out);
    }

    let cache = self.0.borrow();
    let entry_pin = cache.get(&key).unwrap();
    let entry_ref = entry_pin.as_ref().get_ref();

    // SAFETY: because the entry is pinned, it cannot move and this pointer will
    // only be invalidated if Cache is dropped. The returned reference has a lifetime
    // equal to Cache, so Cache cannot be dropped before this reference goes out of scope.
    unsafe { mem::transmute::<&'_ Out, &'a Out>(entry_ref) }
  }
}

impl<In, Out> Default for Cache<In, Out> {
  fn default() -> Self {
    Cache(RefCell::new(HashMap::default()))
  }
}

/// Cache for copyable types.
pub struct CopyCache<In, Out>(RefCell<HashMap<In, Out>>);

impl<In, Out> CopyCache<In, Out>
where
  In: Hash + Eq + Clone,
  Out: Copy,
{
  /// Size of the cache
  pub fn len(&self) -> usize {
    self.0.borrow().len()
  }
  /// Returns the cached value for the given key, or runs `compute` if
  /// the value is not in cache.
  pub fn get(&self, key: In, compute: impl FnOnce(In) -> Out) -> Out {
    let mut cache = self.0.borrow_mut();
    *cache
      .entry(key.clone())
      .or_insert_with(move || compute(key))
  }
}

impl<In, Out> Default for CopyCache<In, Out> {
  fn default() -> Self {
    CopyCache(RefCell::new(HashMap::default()))
  }
}

/// This cache alters the [`Self::get`] method signature to return
/// an [`Option`] of a reference. In particular the method will return [`None`]
/// if it is called *with the same key* while computing a construction function
/// for that key.
pub struct RecursionBreakingCache<In, Out>(RefCell<HashMap<In, Option<Pin<Box<Out>>>>>);

impl<In, Out> RecursionBreakingCache<In, Out>
where
  In: Hash + Eq + Clone,
  Out: Unpin,
{
  /// Size of the cache
  pub fn len(&self) -> usize {
    self.0.borrow().len()
  }
  /// Get or compute the value for this key. Returns `None` if the `compute`
  /// function calls this [`get`] function again *with the same key*. Calls to
  /// [`get`] with different keys are allowed.
  pub fn get<'a>(&'a self, key: In, compute: impl FnOnce(In) -> Out) -> Option<&'a Out> {
    if !self.0.borrow().contains_key(&key) {
      self.0.borrow_mut().insert(key.clone(), None);
      let out = Pin::new(Box::new(compute(key.clone())));
      self.0.borrow_mut().insert(key.clone(), Some(out));
    }

    let cache = self.0.borrow();
    // Important here to first `unwrap` the `Option` created by `get`, then
    // propagate the potential option stored in the map.
    let entry = cache.get(&key).expect("invariant broken").as_ref()?;

    // SAFETY: because the entry is pinned, it cannot move and this pointer will
    // only be invalidated if Cache is dropped. The returned reference has a lifetime
    // equal to Cache, so Cache cannot be dropped before this reference goes out of scope.
    Some(unsafe { std::mem::transmute::<&'_ Out, &'a Out>(&**entry) })
  }
}

impl<In, Out> Default for RecursionBreakingCache<In, Out> {
  fn default() -> Self {
    Self(RefCell::new(HashMap::default()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_cached() {
    let cache: Cache<usize, usize> = Cache::default();
    let x = cache.get(0, |_| 0);
    let y = cache.get(1, |_| 1);
    let z = cache.get(0, |_| 2);
    assert_eq!(*x, 0);
    assert_eq!(*y, 1);
    assert_eq!(*z, 0);
    assert!(std::ptr::eq(x, z));
  }
}
