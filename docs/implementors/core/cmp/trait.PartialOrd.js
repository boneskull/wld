(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;A&gt; PartialOrd&lt;ArrayString&lt;A&gt;&gt; for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A&gt; PartialOrd&lt;str&gt; for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A&gt; PartialOrd&lt;ArrayString&lt;A&gt;&gt; for str <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;CapacityError&lt;T&gt;&gt; for CapacityError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; PartialOrd&lt;ArrayVec&lt;A&gt;&gt; for ArrayVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: PartialOrd,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;BitIdx&lt;T&gt;&gt; for BitIdx&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;BitPos&lt;T&gt;&gt; for BitPos&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;BitMask&lt;T&gt;&gt; for BitMask&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitSlice&lt;C, D&gt;&gt; for BitSlice&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D, '_&gt; PartialOrd&lt;BitSlice&lt;C, D&gt;&gt; for &amp;'_ BitSlice&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D, '_&gt; PartialOrd&lt;&amp;'_ BitSlice&lt;C, D&gt;&gt; for BitSlice&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitBox&lt;C, D&gt;&gt; for BitBox&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitSlice&lt;C, D&gt;&gt; for BitBox&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitBox&lt;C, D&gt;&gt; for BitSlice&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitVec&lt;C, D&gt;&gt; for BitVec&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitSlice&lt;C, D&gt;&gt; for BitVec&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D&gt; PartialOrd&lt;BitVec&lt;C, D&gt;&gt; for BitSlice&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D, '_&gt; PartialOrd&lt;&amp;'_ BitSlice&lt;C, D&gt;&gt; for BitVec&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, C, D, '_&gt; PartialOrd&lt;BitVec&lt;C, D&gt;&gt; for &amp;'_ BitSlice&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["byteorder"] = [{"text":"impl PartialOrd&lt;BigEndian&gt; for BigEndian","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LittleEndian&gt; for LittleEndian","synthetic":false,"types":[]}];
implementors["conv"] = [{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;GeneralError&lt;T&gt;&gt; for GeneralError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;GeneralErrorKind&gt; for GeneralErrorKind","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;NoError&gt; for NoError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;Unrepresentable&lt;T&gt;&gt; for Unrepresentable&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;NegOverflow&lt;T&gt;&gt; for NegOverflow&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;PosOverflow&lt;T&gt;&gt; for PosOverflow&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;FloatError&lt;T&gt;&gt; for FloatError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;RangeError&lt;T&gt;&gt; for RangeError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;RangeErrorKind&gt; for RangeErrorKind","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;'g, T&gt; PartialOrd&lt;Shared&lt;'g, T&gt;&gt; for Shared&lt;'g, T&gt;","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl PartialOrd&lt;Compression&gt; for Compression","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;MatchingType&gt; for MatchingType","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L:&nbsp;PartialOrd, R:&nbsp;PartialOrd&gt; PartialOrd&lt;Either&lt;L, R&gt;&gt; for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["image"] = [{"text":"impl PartialOrd&lt;NormalForm&gt; for NormalForm","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Delay&gt; for Delay","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl PartialOrd&lt;Level&gt; for Level","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LevelFilter&gt; for Level","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LevelFilter&gt; for LevelFilter","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Level&gt; for LevelFilter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;Metadata&lt;'a&gt;&gt; for Metadata&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;MetadataBuilder&lt;'a&gt;&gt; for MetadataBuilder&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl PartialOrd&lt;Sign&gt; for Sign","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BigUint&gt; for BigUint","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; PartialOrd&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["ordered_float"] = [{"text":"impl&lt;T:&nbsp;Float&gt; PartialOrd&lt;OrderedFloat&lt;T&gt;&gt; for OrderedFloat&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;PartialOrd + Float&gt; PartialOrd&lt;NotNan&lt;T&gt;&gt; for NotNan&lt;T&gt;","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl PartialOrd&lt;Transformations&gt; for Transformations","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl PartialOrd&lt;Ident&gt; for Ident","synthetic":false,"types":[]}];
implementors["rusttype"] = [{"text":"impl&lt;N:&nbsp;PartialOrd&gt; PartialOrd&lt;Point&lt;N&gt;&gt; for Point&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;PartialOrd&gt; PartialOrd&lt;Vector&lt;N&gt;&gt; for Vector&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Line&gt; for Line","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Curve&gt; for Curve","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;PartialOrd&gt; PartialOrd&lt;Rect&lt;N&gt;&gt; for Rect&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Codepoint&gt; for Codepoint","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;GlyphId&gt; for GlyphId","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;HMetrics&gt; for HMetrics","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;VMetrics&gt; for VMetrics","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Scale&gt; for Scale","synthetic":false,"types":[]}];
implementors["stb_truetype"] = [{"text":"impl PartialOrd&lt;PlatformId&gt; for PlatformId","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;UnicodeEid&gt; for UnicodeEid","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;MicrosoftEid&gt; for MicrosoftEid","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;MacEid&gt; for MacEid","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;MicrosoftLang&gt; for MicrosoftLang","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;MacLang&gt; for MacLang","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;PlatformEncodingLanguageId&gt; for PlatformEncodingLanguageId","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl PartialOrd&lt;Lifetime&gt; for Lifetime","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl PartialOrd&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Timespec&gt; for Timespec","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;SteadyTime&gt; for SteadyTime","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Tm&gt; for Tm","synthetic":false,"types":[]}];
implementors["uuid"] = [{"text":"impl PartialOrd&lt;Hyphenated&gt; for Hyphenated","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;HyphenatedRef&lt;'a&gt;&gt; for HyphenatedRef&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Simple&gt; for Simple","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;SimpleRef&lt;'a&gt;&gt; for SimpleRef&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Urn&gt; for Urn","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;UrnRef&lt;'a&gt;&gt; for UrnRef&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Uuid&gt; for Uuid","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()