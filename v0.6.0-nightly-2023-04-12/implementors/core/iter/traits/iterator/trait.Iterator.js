(function() {var implementors = {
"addr2line":[["impl&lt;'ctx, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"addr2line/struct.LocationRangeIter.html\" title=\"struct addr2line::LocationRangeIter\">LocationRangeIter</a>&lt;'ctx, R&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"gimli/read/reader/trait.Reader.html\" title=\"trait gimli::read::reader::Reader\">Reader</a> + 'ctx,</span>"]],
"anyhow":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"anyhow/struct.Chain.html\" title=\"struct anyhow::Chain\">Chain</a>&lt;'a&gt;"]],
"camino":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"camino/struct.Utf8Components.html\" title=\"struct camino::Utf8Components\">Utf8Components</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"camino/struct.Utf8Ancestors.html\" title=\"struct camino::Utf8Ancestors\">Utf8Ancestors</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"camino/struct.ReadDirUtf8.html\" title=\"struct camino::ReadDirUtf8\">ReadDirUtf8</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"camino/struct.Iter.html\" title=\"struct camino::Iter\">Iter</a>&lt;'a&gt;"]],
"cargo_metadata":[["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"cargo_metadata/struct.MessageIter.html\" title=\"struct cargo_metadata::MessageIter\">MessageIter</a>&lt;R&gt;"]],
"gimli":[["impl&lt;'index, R: <a class=\"trait\" href=\"gimli/read/trait.Reader.html\" title=\"trait gimli::read::Reader\">Reader</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"gimli/read/struct.UnitIndexSectionIterator.html\" title=\"struct gimli::read::UnitIndexSectionIterator\">UnitIndexSectionIterator</a>&lt;'index, R&gt;"],["impl&lt;'iter, R: <a class=\"trait\" href=\"gimli/read/trait.Reader.html\" title=\"trait gimli::read::Reader\">Reader</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"gimli/read/struct.RegisterRuleIter.html\" title=\"struct gimli::read::RegisterRuleIter\">RegisterRuleIter</a>&lt;'iter, R&gt;"]],
"intervaltree":[["impl&lt;'a, K: 'a, V: 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"intervaltree/struct.TreeIter.html\" title=\"struct intervaltree::TreeIter\">TreeIter</a>&lt;'a, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"intervaltree/struct.TreeIntoIter.html\" title=\"struct intervaltree::TreeIntoIter\">TreeIntoIter</a>&lt;K, V&gt;"],["impl&lt;'a, K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"intervaltree/struct.QueryIter.html\" title=\"struct intervaltree::QueryIter\">QueryIter</a>&lt;'a, K, V&gt;"]],
"memchr":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr2.html\" title=\"struct memchr::Memchr2\">Memchr2</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr.html\" title=\"struct memchr::Memchr\">Memchr</a>&lt;'a&gt;"],["impl&lt;'h, 'n&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"memchr/memmem/struct.FindIter.html\" title=\"struct memchr::memmem::FindIter\">FindIter</a>&lt;'h, 'n&gt;"],["impl&lt;'h, 'n&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"memchr/memmem/struct.FindRevIter.html\" title=\"struct memchr::memmem::FindRevIter\">FindRevIter</a>&lt;'h, 'n&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr3.html\" title=\"struct memchr::Memchr3\">Memchr3</a>&lt;'a&gt;"]],
"object":[["impl&lt;'data, 'file, Mach, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.MachOSymbolIterator.html\" title=\"struct object::read::macho::MachOSymbolIterator\">MachOSymbolIterator</a>&lt;'data, 'file, Mach, R&gt;<span class=\"where fmt-newline\">where\n    Mach: <a class=\"trait\" href=\"object/read/macho/trait.MachHeader.html\" title=\"trait object::read::macho::MachHeader\">MachHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.DynamicRelocationIterator.html\" title=\"struct object::read::DynamicRelocationIterator\">DynamicRelocationIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/pe/struct.RelocationIterator.html\" title=\"struct object::read::pe::RelocationIterator\">RelocationIterator</a>&lt;'data&gt;"],["impl&lt;'data, 'file, Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfSymbolIterator.html\" title=\"struct object::read::elf::ElfSymbolIterator\">ElfSymbolIterator</a>&lt;'data, 'file, Elf, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffRelocationIterator.html\" title=\"struct object::read::coff::CoffRelocationIterator\">CoffRelocationIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffComdatSectionIterator.html\" title=\"struct object::read::coff::CoffComdatSectionIterator\">CoffComdatSectionIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.ComdatIterator.html\" title=\"struct object::read::ComdatIterator\">ComdatIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, Elf, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfComdatSectionIterator.html\" title=\"struct object::read::elf::ElfComdatSectionIterator\">ElfComdatSectionIterator</a>&lt;'data, 'file, Elf, R&gt;<span class=\"where fmt-newline\">where\n    Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Pe, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/pe/struct.PeSectionIterator.html\" title=\"struct object::read::pe::PeSectionIterator\">PeSectionIterator</a>&lt;'data, 'file, Pe, R&gt;<span class=\"where fmt-newline\">where\n    Pe: <a class=\"trait\" href=\"object/read/pe/trait.ImageNtHeaders.html\" title=\"trait object::read::pe::ImageNtHeaders\">ImageNtHeaders</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Mach, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.MachORelocationIterator.html\" title=\"struct object::read::macho::MachORelocationIterator\">MachORelocationIterator</a>&lt;'data, 'file, Mach, R&gt;<span class=\"where fmt-newline\">where\n    Mach: <a class=\"trait\" href=\"object/read/macho/trait.MachHeader.html\" title=\"trait object::read::macho::MachHeader\">MachHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Elf, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfSectionRelocationIterator.html\" title=\"struct object::read::elf::ElfSectionRelocationIterator\">ElfSectionRelocationIterator</a>&lt;'data, 'file, Elf, R&gt;<span class=\"where fmt-newline\">where\n    Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffSymbolIterator.html\" title=\"struct object::read::coff::CoffSymbolIterator\">CoffSymbolIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffSectionIterator.html\" title=\"struct object::read::coff::CoffSectionIterator\">CoffSectionIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, Pe, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/pe/struct.PeComdatIterator.html\" title=\"struct object::read::pe::PeComdatIterator\">PeComdatIterator</a>&lt;'data, 'file, Pe, R&gt;<span class=\"where fmt-newline\">where\n    Pe: <a class=\"trait\" href=\"object/read/pe/trait.ImageNtHeaders.html\" title=\"trait object::read::pe::ImageNtHeaders\">ImageNtHeaders</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Pe, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/pe/struct.PeSegmentIterator.html\" title=\"struct object::read::pe::PeSegmentIterator\">PeSegmentIterator</a>&lt;'data, 'file, Pe, R&gt;<span class=\"where fmt-newline\">where\n    Pe: <a class=\"trait\" href=\"object/read/pe/trait.ImageNtHeaders.html\" title=\"trait object::read::pe::ImageNtHeaders\">ImageNtHeaders</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.NoDynamicRelocationIterator.html\" title=\"struct object::read::NoDynamicRelocationIterator\">NoDynamicRelocationIterator</a>"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.SectionIterator.html\" title=\"struct object::read::SectionIterator\">SectionIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.ComdatSectionIterator.html\" title=\"struct object::read::ComdatSectionIterator\">ComdatSectionIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, Elf, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfSectionIterator.html\" title=\"struct object::read::elf::ElfSectionIterator\">ElfSectionIterator</a>&lt;'data, 'file, Elf, R&gt;<span class=\"where fmt-newline\">where\n    Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.SegmentIterator.html\" title=\"struct object::read::SegmentIterator\">SegmentIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, Mach, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.MachOComdatIterator.html\" title=\"struct object::read::macho::MachOComdatIterator\">MachOComdatIterator</a>&lt;'data, 'file, Mach, R&gt;<span class=\"where fmt-newline\">where\n    Mach: <a class=\"trait\" href=\"object/read/macho/trait.MachHeader.html\" title=\"trait object::read::macho::MachHeader\">MachHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/archive/struct.ArchiveMemberIterator.html\" title=\"struct object::read::archive::ArchiveMemberIterator\">ArchiveMemberIterator</a>&lt;'data, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffSegmentIterator.html\" title=\"struct object::read::coff::CoffSegmentIterator\">CoffSegmentIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.SymbolIterator.html\" title=\"struct object::read::SymbolIterator\">SymbolIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, Mach, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.MachOComdatSectionIterator.html\" title=\"struct object::read::macho::MachOComdatSectionIterator\">MachOComdatSectionIterator</a>&lt;'data, 'file, Mach, R&gt;<span class=\"where fmt-newline\">where\n    Mach: <a class=\"trait\" href=\"object/read/macho/trait.MachHeader.html\" title=\"trait object::read::macho::MachHeader\">MachHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'cache, E, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.DyldCacheImageIterator.html\" title=\"struct object::read::macho::DyldCacheImageIterator\">DyldCacheImageIterator</a>&lt;'data, 'cache, E, R&gt;<span class=\"where fmt-newline\">where\n    E: <a class=\"trait\" href=\"object/endian/trait.Endian.html\" title=\"trait object::endian::Endian\">Endian</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/struct.SectionRelocationIterator.html\" title=\"struct object::read::SectionRelocationIterator\">SectionRelocationIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, Mach, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.MachOSegmentIterator.html\" title=\"struct object::read::macho::MachOSegmentIterator\">MachOSegmentIterator</a>&lt;'data, 'file, Mach, R&gt;<span class=\"where fmt-newline\">where\n    Mach: <a class=\"trait\" href=\"object/read/macho/trait.MachHeader.html\" title=\"trait object::read::macho::MachHeader\">MachHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Elf, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfComdatIterator.html\" title=\"struct object::read::elf::ElfComdatIterator\">ElfComdatIterator</a>&lt;'data, 'file, Elf, R&gt;<span class=\"where fmt-newline\">where\n    Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Mach, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/macho/struct.MachOSectionIterator.html\" title=\"struct object::read::macho::MachOSectionIterator\">MachOSectionIterator</a>&lt;'data, 'file, Mach, R&gt;<span class=\"where fmt-newline\">where\n    Mach: <a class=\"trait\" href=\"object/read/macho/trait.MachHeader.html\" title=\"trait object::read::macho::MachHeader\">MachHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffComdatIterator.html\" title=\"struct object::read::coff::CoffComdatIterator\">CoffComdatIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'file, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/pe/struct.PeRelocationIterator.html\" title=\"struct object::read::pe::PeRelocationIterator\">PeRelocationIterator</a>&lt;'data, 'file, R&gt;"],["impl&lt;'data, 'table, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/coff/struct.SymbolIterator.html\" title=\"struct object::read::coff::SymbolIterator\">SymbolIterator</a>&lt;'data, 'table, R&gt;"],["impl&lt;'data, 'file, Pe, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/pe/struct.PeComdatSectionIterator.html\" title=\"struct object::read::pe::PeComdatSectionIterator\">PeComdatSectionIterator</a>&lt;'data, 'file, Pe, R&gt;<span class=\"where fmt-newline\">where\n    Pe: <a class=\"trait\" href=\"object/read/pe/trait.ImageNtHeaders.html\" title=\"trait object::read::pe::ImageNtHeaders\">ImageNtHeaders</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Elf, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfDynamicRelocationIterator.html\" title=\"struct object::read::elf::ElfDynamicRelocationIterator\">ElfDynamicRelocationIterator</a>&lt;'data, 'file, Elf, R&gt;<span class=\"where fmt-newline\">where\n    Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"],["impl&lt;'data, 'file, Elf, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"object/read/elf/struct.ElfSegmentIterator.html\" title=\"struct object::read::elf::ElfSegmentIterator\">ElfSegmentIterator</a>&lt;'data, 'file, Elf, R&gt;<span class=\"where fmt-newline\">where\n    Elf: <a class=\"trait\" href=\"object/read/elf/trait.FileHeader.html\" title=\"trait object::read::elf::FileHeader\">FileHeader</a>,\n    R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;,</span>"]],
"proc_macro2":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"proc_macro2/token_stream/struct.IntoIter.html\" title=\"struct proc_macro2::token_stream::IntoIter\">IntoIter</a>"]],
"serde_json":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.Values.html\" title=\"struct serde_json::map::Values\">Values</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.ValuesMut.html\" title=\"struct serde_json::map::ValuesMut\">ValuesMut</a>&lt;'a&gt;"],["impl&lt;'de, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/struct.StreamDeserializer.html\" title=\"struct serde_json::StreamDeserializer\">StreamDeserializer</a>&lt;'de, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"serde_json/de/trait.Read.html\" title=\"trait serde_json::de::Read\">Read</a>&lt;'de&gt;,\n    T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</span>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.Keys.html\" title=\"struct serde_json::map::Keys\">Keys</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.IterMut.html\" title=\"struct serde_json::map::IterMut\">IterMut</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.Iter.html\" title=\"struct serde_json::map::Iter\">Iter</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.IntoIter.html\" title=\"struct serde_json::map::IntoIter\">IntoIter</a>"]],
"smallvec":[["impl&lt;A: <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"smallvec/struct.IntoIter.html\" title=\"struct smallvec::IntoIter\">IntoIter</a>&lt;A&gt;"],["impl&lt;'a, T: 'a + <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"smallvec/struct.Drain.html\" title=\"struct smallvec::Drain\">Drain</a>&lt;'a, T&gt;"]],
"syn":[["impl&lt;T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IntoPairs.html\" title=\"struct syn::punctuated::IntoPairs\">IntoPairs</a>&lt;T, P&gt;"],["impl&lt;'a, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.Pairs.html\" title=\"struct syn::punctuated::Pairs\">Pairs</a>&lt;'a, T, P&gt;"],["impl&lt;'a, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.PairsMut.html\" title=\"struct syn::punctuated::PairsMut\">PairsMut</a>&lt;'a, T, P&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.Iter.html\" title=\"struct syn::punctuated::Iter\">Iter</a>&lt;'a, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IntoIter.html\" title=\"struct syn::punctuated::IntoIter\">IntoIter</a>&lt;T&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IterMut.html\" title=\"struct syn::punctuated::IterMut\">IterMut</a>&lt;'a, T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()