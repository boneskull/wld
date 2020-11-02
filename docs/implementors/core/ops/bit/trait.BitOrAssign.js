(function() {var implementors = {};
implementors["bitvec"] = [{"text":"impl&lt;O, T, I&gt; BitOrAssign&lt;I&gt; for BitSlice&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: IntoIterator&lt;Item = bool&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, I&gt; BitOrAssign&lt;I&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: IntoIterator&lt;Item = bool&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, I&gt; BitOrAssign&lt;I&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: IntoIterator&lt;Item = bool&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl BitOrAssign&lt;BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BitOrAssign&lt;&amp;'a BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl BitOrAssign&lt;BigUint&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BitOrAssign&lt;&amp;'a BigUint&gt; for BigUint","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl BitOrAssign&lt;Transformations&gt; for Transformations","synthetic":false,"types":[]}];
implementors["rulinalg"] = [{"text":"impl&lt;T:&nbsp;Copy + BitOr&lt;T, Output = T&gt;&gt; BitOrAssign&lt;T&gt; for Vector&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Copy + BitOr&lt;T, Output = T&gt;&gt; BitOrAssign&lt;&amp;'a T&gt; for Vector&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + BitOr&lt;T, Output = T&gt;&gt; BitOrAssign&lt;Vector&lt;T&gt;&gt; for Vector&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Copy + BitOr&lt;T, Output = T&gt;&gt; BitOrAssign&lt;&amp;'a Vector&lt;T&gt;&gt; for Vector&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()