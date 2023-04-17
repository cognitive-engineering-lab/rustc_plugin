use std::cmp;

use log::trace;
use rustc_middle::ty::TyCtxt;
use rustc_span::{source_map::SourceMap, BytePos, Pos, Span, SpanData, SyntaxContext};

/// Extension trait for [`Span`].
pub trait SpanExt {
  /// Get spans for regions in `self` not in `child_spans`.
  ///
  /// For example:
  /// ```text
  /// self:          ---------------
  /// child_spans:    ---      --  -
  /// output:        -   ------  --
  /// ```
  fn subtract(&self, child_spans: Vec<Span>) -> Vec<Span>;

  /// Gets the version of this span that is local to the current
  /// crate, and must be contained in `outer_span`.
  fn as_local(&self, outer_span: Span) -> Option<Span>;

  /// Returns true if `self` overlaps with `other` including boundaries.
  fn overlaps_inclusive(&self, other: Span) -> bool;

  /// Returns a new span whose end is no later than the start of `other`,
  /// returning `None` if this would return an empty span.
  fn trim_end(&self, other: Span) -> Option<Span>;

  /// Merges all overlapping spans in the input vector into single spans.
  fn merge_overlaps(spans: Vec<Span>) -> Vec<Span>;

  /// Returns a collection of spans inside `self` that have leading whitespace removed.
  ///
  /// Returns `None` if [`SourceMap::span_to_snippet`] fails.
  fn trim_leading_whitespace(&self, source_map: &SourceMap) -> Option<Vec<Span>>;

  fn to_string(&self, tcx: TyCtxt<'_>) -> String;
  fn size(&self) -> u32;
}

impl SpanExt for Span {
  fn trim_end(&self, other: Span) -> Option<Span> {
    let span = self.data();
    let other = other.data();
    if span.lo < other.lo {
      Some(span.with_hi(cmp::min(span.hi, other.lo)))
    } else {
      None
    }
  }

  fn subtract(&self, mut child_spans: Vec<Span>) -> Vec<Span> {
    child_spans.retain(|s| s.overlaps_inclusive(*self));

    let mut outer_spans = vec![];
    if !child_spans.is_empty() {
      // Output will be sorted
      child_spans = Span::merge_overlaps(child_spans);

      if let Some(start) = self.trim_end(*child_spans.first().unwrap()) {
        outer_spans.push(start);
      }

      for children in child_spans.windows(2) {
        outer_spans.push(children[0].between(children[1]));
      }

      if let Some(end) = self.trim_start(*child_spans.last().unwrap()) {
        outer_spans.push(end);
      }
    } else {
      outer_spans.push(*self);
    };

    trace!("outer span for {self:?} with inner spans {child_spans:?} is {outer_spans:?}");

    outer_spans
  }

  fn as_local(&self, outer_span: Span) -> Option<Span> {
    // Before we call source_callsite, we check and see if the span is already local.
    // This is important b/c in print!("{}", y) if the user selects `y`, the source_callsite
    // of that span is the entire macro.
    if outer_span.contains(*self) {
      return Some(*self);
    } else {
      let sp = self.source_callsite();
      if outer_span.contains(sp) {
        return Some(sp);
      }
    }

    None
  }

  fn overlaps_inclusive(&self, other: Span) -> bool {
    let s1 = self.data();
    let s2 = other.data();
    s1.lo <= s2.hi && s2.lo <= s1.hi
  }

  fn merge_overlaps(mut spans: Vec<Span>) -> Vec<Span> {
    spans.sort_by_key(|s| (s.lo(), s.hi()));

    // See note in Span::subtract
    for span in spans.iter_mut() {
      *span = span.with_ctxt(SyntaxContext::root());
    }

    let mut output = Vec::new();
    for span in spans {
      match output
        .iter_mut()
        .find(|other| span.overlaps_inclusive(**other))
      {
        Some(other) => {
          *other = span.to(*other);
        }
        None => {
          output.push(span);
        }
      }
    }
    output
  }

  fn to_string(&self, tcx: TyCtxt<'_>) -> String {
    let source_map = tcx.sess.source_map();
    let lo = source_map.lookup_char_pos(self.lo());
    let hi = source_map.lookup_char_pos(self.hi());
    let snippet = source_map.span_to_snippet(*self).unwrap();
    format!(
      "{snippet} ({}:{}-{}:{})",
      lo.line,
      lo.col.to_usize() + 1,
      hi.line,
      hi.col.to_usize() + 1
    )
  }

  fn size(&self) -> u32 {
    self.hi().0 - self.lo().0
  }

  fn trim_leading_whitespace(&self, source_map: &SourceMap) -> Option<Vec<Span>> {
    let snippet = source_map.span_to_snippet(*self).ok()?;
    let mut spans = Vec::new();
    let mut start = self.lo();
    for line in snippet.split('\n') {
      let offset = line
        .chars()
        .take_while(|c| c.is_whitespace())
        .map(|c| c.len_utf8())
        .sum::<usize>();
      let end = (start + BytePos(line.len() as u32)).min(self.hi());
      spans.push(self.with_lo(start + BytePos(offset as u32)).with_hi(end));
      start = end + BytePos(1);
    }
    Some(spans)
  }
}

/// Extension trait for [`SpanData`].
pub trait SpanDataExt {
  fn size(&self) -> u32;
}

impl SpanDataExt for SpanData {
  fn size(&self) -> u32 {
    self.hi.0 - self.lo.0
  }
}

#[cfg(test)]
mod test {
  use rustc_span::BytePos;

  use super::*;

  #[test]
  fn test_span_subtract() {
    rustc_span::create_default_session_if_not_set_then(|_| {
      let mk = |lo, hi| Span::with_root_ctxt(BytePos(lo), BytePos(hi));
      let outer = mk(1, 10);
      let inner: Vec<Span> = vec![mk(0, 2), mk(3, 4), mk(3, 5), mk(7, 8), mk(9, 13)];
      let desired: Vec<Span> = vec![mk(2, 3), mk(5, 7), mk(8, 9)];
      assert_eq!(outer.subtract(inner), desired);
    });
  }
}
