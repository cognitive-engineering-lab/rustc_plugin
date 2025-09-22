use intervaltree::IntervalTree;
use rustc_span::{BytePos, SpanData, source_map::Spanned};

/// Interval tree data structure specialized to spans.
pub struct SpanTree<T> {
  tree: IntervalTree<BytePos, (SpanData, T)>,
  len: usize,
}

impl<T> SpanTree<T> {
  pub fn new(spans: impl IntoIterator<Item = Spanned<T>>) -> Self {
    let tree = spans
      .into_iter()
      .map(|spanned| {
        let data = spanned.span.data();
        (data.lo .. data.hi, (data, spanned.node))
      })
      .collect::<IntervalTree<_, _>>();
    let len = tree.iter().count();
    SpanTree { tree, len }
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn iter(&self) -> impl Iterator<Item = &'_ T> + '_ {
    self.tree.iter().map(|el| &el.value.1)
  }

  /// Find all spans that overlap with `query`
  pub fn overlapping(
    &self,
    query: SpanData,
  ) -> impl Iterator<Item = &'_ (SpanData, T)> + '_ {
    self.tree.query(query.lo .. query.hi).map(|el| &el.value)
  }
}

#[cfg(test)]
mod test {
  use rustc_span::SyntaxContext;

  use super::*;

  #[test]
  fn span_tree_test() {
    rustc_span::create_default_session_globals_then(|| {
      let mk_span = |lo, hi| SpanData {
        lo: BytePos(lo),
        hi: BytePos(hi),
        ctxt: SyntaxContext::root(),
        parent: None,
      };
      let mk = |node, lo, hi| Spanned {
        span: mk_span(lo, hi).span(),
        node,
      };

      let input = [mk("a", 0, 1), mk("b", 2, 3), mk("c", 0, 5)];
      let tree = SpanTree::new(input);

      let query = |lo, hi| {
        let mut result = tree
          .overlapping(mk_span(lo, hi))
          .map(|(_, t)| t)
          .copied()
          .collect::<Vec<_>>();
        result.sort_unstable();
        result
      };

      assert_eq!(query(0, 2), ["a", "c"]);
      assert_eq!(query(0, 3), ["a", "b", "c"]);
      assert_eq!(query(2, 3), ["b", "c"]);
      assert_eq!(query(6, 8), [] as [&str; 0]);
    });
  }
}
