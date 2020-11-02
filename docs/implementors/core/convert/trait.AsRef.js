(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;A&gt; AsRef&lt;str&gt; for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; AsRef&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;O, T, '_&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for Iter&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, '_&gt; AsRef&lt;[T]&gt; for Iter&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;[T]&gt; for BitSlice&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;[T]&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;[T]&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["byteio"] = [{"text":"impl&lt;'a, R:&nbsp;ReadBytes&lt;'a&gt;&gt; AsRef&lt;[u8]&gt; for Reader&lt;'a, R&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; AsRef&lt;T&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R, Target&gt; AsRef&lt;Target&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;Target&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;Target&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;L, R&gt; AsRef&lt;str&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;str&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;str&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;L, R, Target&gt; AsRef&lt;[Target]&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;[Target]&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;[Target]&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["failure"] = [{"text":"impl AsRef&lt;dyn Fail + 'static&gt; for Error","synthetic":false,"types":[]}];
implementors["ordered_float"] = [{"text":"impl&lt;T:&nbsp;Float&gt; AsRef&lt;T&gt; for OrderedFloat&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Float&gt; AsRef&lt;T&gt; for NotNan&lt;T&gt;","synthetic":false,"types":[]}];
implementors["scroll"] = [{"text":"impl AsRef&lt;u64&gt; for Uleb128","synthetic":false,"types":[]},{"text":"impl AsRef&lt;i64&gt; for Sleb128","synthetic":false,"types":[]}];
implementors["wld"] = [{"text":"impl AsRef&lt;BitVec&lt;Lsb0, u8&gt;&gt; for TBitVec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;House&gt;&gt; for HouseVec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;ItemStack&gt;&gt; for ItemStackVec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;Mob&gt;&gt; for MobVec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;NPC&gt;&gt; for NPCVec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;Position&gt;&gt; for PressurePlates","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Uuid&gt; for TUuid","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;i32&gt;&gt; for MobKills","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;i32&gt;&gt; for PartyingNPCs","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;Tile&gt;&gt; for TileVec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;TileVec&gt;&gt; for TileMatrix","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Vec&lt;u8&gt;&gt; for TownManager","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()