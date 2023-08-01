//! Data structures for memoizing computations.
//!
//! Contruct new caches using [`Default::default`], then construct/retrieve
//! elements with [`get`](Cache::get). `get` should only ever be used with one,
//! `compute` function[^inconsistent].
//!
//! In terms of choice,
//! - [`CopyCache`] should be used for expensive computations that create cheap
//!   (i.e. small) values.
//! - [`Cache`] should be used for expensive computations that create expensive
//!   (i.e. large) values.
//!
//! Both types of caches implement **recursion breaking**. In general because
//! caches are supposed to be used as simple `&` (no `mut`) the reference may be
//! freely copied, including into the `compute` closure. What this means is that
//! a `compute` may call [`get`](Cache::get) on the cache again. This is usually
//! safe and can be used to compute data structures that recursively depend on
//! one another, dynamic-programming style. However if a `get` on a key `k`
//! itself calls `get` again on the same `k` this will either create an infinite
//! recursion or an inconsistent cache[^inconsistent].
//!
//! Consider a simple example where we compute the Fibonacci Series with a
//! [`CopyCache`]:
//!
//! ```rs
//! let cache = CopyCache::default();
//! let next_fib = |this| {
//!   if this <= 1 { return this; }
//!   let fib_1 = cache.get(this - 1, next_fib);
//!   let fib_2 = cache.get(this - 2, next_fib);
//!   fib_1 + fib_2
//! };
//! let fib_5 = cache.get(5, next_fib);
//! ```
//!
//! This use of recursive [`get`](CopyCache::get) calls is perfectly legal.
//! However if we made an error and called `chache.get(this, ...)` (forgetting
//! the decrement) we would have created an inadvertend infinite recursion.
//!
//! To avoid this scenario both caches are implemented to detect when a
//! recursive call as described is performed and `get` will panic. If your code
//! uses recursive construction and would like to handle this case gracefully
//! use [`get_maybe_recursive`](Cache::get_maybe_recursive) instead wich returns
//! `None` from `get(k)` *iff* `k` this call (potentially transitively)
//! originates from another `get(k)` call.
//!
//! [^inconsistent]: For any given cache value `get` should only ever be used
//!     with one, referentially transparent `compute` function. Essentially this
//!     means running `compute(k)` should always return the same value
//!     *independent of the state of it's environment*. Violation of this rule
//!     can introduces non-determinism in your program.
use std::{cell::RefCell, hash::Hash, pin::Pin};

use rustc_data_structures::fx::FxHashMap as HashMap;

/// Cache for non-copyable types.
pub struct Cache<In, Out>(RefCell<HashMap<In, Option<Pin<Box<Out>>>>>);

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
  ///
  /// # Panics
  ///
  /// Returns `None` if this is a recursive invocation of `get` for key `key`.
  pub fn get<'a>(&'a self, key: In, compute: impl FnOnce(In) -> Out) -> &'a Out {
    self
      .get_maybe_recursive(key, compute)
      .unwrap_or_else(recursion_panic)
  }
  /// Returns the cached value for the given key, or runs `compute` if
  /// the value is not in cache.
  ///
  /// Returns `None` if this is a recursive invocation of `get` for key `key`.
  pub fn get_maybe_recursive<'a>(
    &'a self,
    key: In,
    compute: impl FnOnce(In) -> Out,
  ) -> Option<&'a Out> {
    if !self.0.borrow().contains_key(&key) {
      self.0.borrow_mut().insert(key.clone(), None);
      let out = Box::pin(compute(key.clone()));
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

fn recursion_panic<A>() -> A {
  panic!("Recursion detected! The computation of a value tried to retrieve the same from the cache. Using `get_maybe_recursive` to handle this case gracefully.")
}

impl<In, Out> Default for Cache<In, Out> {
  fn default() -> Self {
    Cache(RefCell::new(HashMap::default()))
  }
}

/// Cache for copyable types.
pub struct CopyCache<In, Out>(RefCell<HashMap<In, Option<Out>>>);

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
  ///
  /// # Panics
  ///
  /// Returns `None` if this is a recursive invocation of `get` for key `key`.
  pub fn get(&self, key: In, compute: impl FnOnce(In) -> Out) -> Out {
    self
      .get_maybe_recursive(key, compute)
      .unwrap_or_else(recursion_panic)
  }

  /// Returns the cached value for the given key, or runs `compute` if
  /// the value is not in cache.
  ///
  /// Returns `None` if this is a recursive invocation of `get` for key `key`.
  pub fn get_maybe_recursive(
    &self,
    key: In,
    compute: impl FnOnce(In) -> Out,
  ) -> Option<Out> {
    if !self.0.borrow().contains_key(&key) {
      self.0.borrow_mut().insert(key.clone(), None);
      let out = compute(key.clone());
      self.0.borrow_mut().insert(key.clone(), Some(out));
    }

    *self.0.borrow_mut().get(&key).expect("invariant broken")
  }
}

impl<In, Out> Default for CopyCache<In, Out> {
  fn default() -> Self {
    CopyCache(RefCell::new(HashMap::default()))
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
