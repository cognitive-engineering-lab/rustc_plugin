searchState.loadedDescShard("rustc_utils", 0, "<code>rustc_utils</code> provides a wide variety of utilities for …\nLogs the time taken from the start to the end of a …\nData structures for memoizing computations.\nUtility for hashset literals. Same as maplit::hashset but …\nUtilities for HIR-level data structures.\nUtilities for MIR-level data structures.\nUtilities for source-mapping text ranges to program …\nRunning rustc and Flowistry in tests.\nA simple timer for profiling.\nCache for non-copyable types.\nCache for copyable types.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the cached value for the given key, or runs <code>compute</code>…\nReturns the cached value for the given key, or runs <code>compute</code>…\nReturns the cached value for the given key, or runs <code>compute</code>…\nReturns the cached value for the given key, or runs <code>compute</code>…\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSize of the cache\nSize of the cache\nUtilities for <code>Ty</code>.\nExtension trait for <code>Ty</code>.\nReturns true if a type implements a given trait.\nReturns an iterator over the regions appearing within a …\nReturns true if a type implements <code>Copy</code>.\nUtilities for <code>AdtDef</code>.\nUtilities for <code>Body</code>.\nPolonius integration to extract borrowck facts from rustc.\nAn algorithm to compute control-dependencies between MIR …\nUtilities for <code>Mutability</code>.\nUtilities for <code>Operand</code>.\nUtilities for <code>Place</code>.\nExtension trait for <code>AdtDef</code>.\nReturns an iterator over all the fields of the ADT that …\nExtension trait for <code>Body</code>.\nReturns an iterator over all the locations in a body.\nReturns an iterator over all projections of all local …\nReturns an iterator over the locations of …\nIf this body is an async function, then return the type of …\nReturns all the control dependencies within the CFG.\nReturns a mapping from source-level variable names to <code>Local</code>…\nReturns the <code>HirId</code> corresponding to a MIR <code>Location</code>.\nReturns all the locations in a <code>BasicBlock</code>.\nReturns an iterator over all the regions that appear in …\nReturns an iterator over all the regions that appear in …\nConverts a Body to a debug representation.\nMIR pass to remove instructions not important for …\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nGets the MIR body and Polonius-generated borrowck facts …\nCalls <code>U::from(self)</code>.\nYou must use this function in …\nRepresents the control dependencies between all pairs of …\nRepresents the post-dominators of a graph’s nodes with …\nConstructs the post-dominators by computing the dominators …\nCompute the union of control dependencies from multiple …\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the set of all node that are control-dependent on …\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets the node that immediately post-dominators <code>node</code>, if …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGets all nodes that post-dominate <code>node</code>, if they exist.\nUsed to represent dependencies of places.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nDoes this index type assert if asked to construct an index …\nIf <code>Self::CHECKS_MAX_INDEX</code> is true, we’ll assert if …\nAsserts <code>v &lt;= Self::MAX_INDEX</code> unless Self::CHECKS_MAX_INDEX …\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct this index type from one in a different domain\nConstruct this index type from the wrapped integer type.\nConstruct from the underlying type without any checks.\nConstruct this index type from a usize.\nConstruct from a usize without any checks.\nGet the wrapped index as a usize.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstruct this index type from a usize. Alias for …\nGet the wrapped index.\nReturns true if <code>self</code> is equally or more permissive than …\nExtension trait for <code>Operand</code>.\nExtracts the <code>Place</code> inside an <code>Operand</code> if it exists.\nA MIR <code>Visitor</code> which collects all <code>Place</code>s that appear in the …\nExtension trait for <code>Place</code>.\nMIR pass to remove instructions not important for …\nUsed to describe aliases of owned and raw pointers.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a new <code>Place</code> with an empty projection.\nConverts a <code>PlaceRef</code> into an owned <code>Place</code>.\nReturns all possible projections of <code>self</code>.\nReturns all possible projections of <code>self</code> that do not go …\nReturns all possible projections of <code>self</code> that are …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if <code>self</code> is a projection of an argument local.\nReturns true if <code>self</code> could not be resolved further to …\nReturns true if this place’s base <code>Local</code> corresponds to …\nCreates a new <code>Place</code>.\nErases/normalizes information in a place to ensure stable …\nReturns an iterator over all prefixes of <code>self</code>’s …\nReturns a pretty representation of a place that uses debug …\nMapping source ranges to/from the HIR and MIR.\nMaximum value the index can take.\nMaximum value the index can take, as a <code>u32</code>.\nZero value of the index.\nExtracts the value of this index as a <code>u32</code>.\nExtracts the value of this index as a <code>usize</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a new index from a given <code>u32</code>.\nCreates a new index from a given <code>u32</code>.\nCreates a new index from a given <code>usize</code>.\nExtracts the value of this index as a <code>usize</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nFinds all bodies in the current crate\nFinds all the bodies that enclose the given span, from …\nCharPos is designed to match VSCode’s vscode.Position …\nData structure for sharing spans outside rustc.\nAn externally-provided identifier of a function\nName of a function\nRange of code possibly inside a function\nUsed to convert objects into a <code>Span</code> with access to <code>TyCtxt</code>\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExtension trait for <code>SpanData</code>.\nExtension trait for <code>Span</code>.\nReturns the version of this span that is local to the …\nMerges all overlapping spans in the input vector into …\nReturns true if <code>self</code> overlaps with <code>other</code> including …\nReturns the size (in bytes) of the spanned text.\nReturns the size (in bytes) of the spanned text.\nReturns spans for regions in <code>self</code> not in <code>child_spans</code>.\nReturns a pretty debug representation of a span.\nReturns a new span whose end is no later than the start of …\nReturns a collection of spans inside <code>self</code> that have …\nWhich parts of a HIR node’s span should be included for …\nThe entire span\nNo span\nThe spans of the node minus its children\nConverts MIR locations to source spans using HIR …\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nEquivalent to <code>f(&amp;iter.collect::&lt;Vec&lt;_&gt;&gt;())</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.")