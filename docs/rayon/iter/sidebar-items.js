initSidebarItems({"enum":[["Either","The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases."]],"fn":[["empty","Creates a parallel iterator that produces nothing."],["once","Creates a parallel iterator that produces an element exactly once."],["repeat","Creates a parallel iterator that endlessly repeats `elt` (by cloning it). Note that this iterator has \"infinite\" length, so typically you would want to use `zip` or `take` or some other means to shorten it, or consider using the `repeatn()` function instead."],["repeatn","Creates a parallel iterator that produces `n` repeats of `elt` (by cloning it)."],["split","The `split` function takes arbitrary data and a closure that knows how to split it, and turns this into a `ParallelIterator`."]],"mod":[["plumbing","Traits and functions used to implement parallel iteration.  These are low-level details -- users of parallel iterators should not need to interact with them directly.  See the `plumbing` README for a high-level overview."]],"struct":[["Chain","`Chain` is an iterator that joins `b` after `a` in one continuous iterator. This struct is created by the `chain()` method on `ParallelIterator`"],["Chunks","`Chunks` is an iterator that groups elements of an underlying iterator."],["Cloned","`Cloned` is an iterator that clones the elements of an underlying iterator."],["Copied","`Copied` is an iterator that copies the elements of an underlying iterator."],["Empty","Iterator adaptor for the `empty()` function."],["Enumerate","`Enumerate` is an iterator that returns the current count along with the element. This struct is created by the `enumerate()` method on `IndexedParallelIterator`"],["Filter","`Filter` takes a predicate `filter_op` and filters out elements that match. This struct is created by the `filter()` method on `ParallelIterator`"],["FilterMap","`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements. This struct is created by the `filter_map()` method on `ParallelIterator`."],["FlatMap","`FlatMap` maps each element to an iterator, then flattens these iterators together. This struct is created by the `flat_map()` method on `ParallelIterator`"],["Flatten","`Flatten` turns each element to an iterator, then flattens these iterators together. This struct is created by the `flatten()` method on `ParallelIterator`."],["Fold","`Fold` is an iterator that applies a function over an iterator producing a single value. This struct is created by the `fold()` method on `ParallelIterator`"],["FoldWith","`FoldWith` is an iterator that applies a function over an iterator producing a single value. This struct is created by the `fold_with()` method on `ParallelIterator`"],["Inspect","`Inspect` is an iterator that calls a function with a reference to each element before yielding it."],["Interleave","`Interleave` is an iterator that interleaves elements of iterators `i` and `j` in one continuous iterator. This struct is created by the `interleave()` method on `IndexedParallelIterator`"],["InterleaveShortest","`InterleaveShortest` is an iterator that works similarly to `Interleave`, but this version stops returning elements once one of the iterators run out."],["Intersperse","`Intersperse` is an iterator that inserts a particular item between each item of the adapted iterator.  This struct is created by the `intersperse()` method on `ParallelIterator`"],["IterBridge","`IterBridge` is a parallel iterator that wraps a sequential iterator."],["Map","`Map` is an iterator that transforms the elements of an underlying iterator."],["MapInit","`MapInit` is an iterator that transforms the elements of an underlying iterator."],["MapWith","`MapWith` is an iterator that transforms the elements of an underlying iterator."],["MaxLen","`MaxLen` is an iterator that imposes a maximum length on iterator splits. This struct is created by the `max_len()` method on `IndexedParallelIterator`"],["MinLen","`MinLen` is an iterator that imposes a minimum length on iterator splits. This struct is created by the `min_len()` method on `IndexedParallelIterator`"],["MultiZip","`MultiZip` is an iterator that zips up a tuple of parallel iterators to produce tuples of their items."],["Once","Iterator adaptor for the `once()` function."],["PanicFuse","`PanicFuse` is an adaptor that wraps an iterator with a fuse in case of panics, to halt all threads as soon as possible."],["Repeat","Iterator adaptor for the `repeat()` function."],["RepeatN","Iterator adaptor for the `repeatn()` function."],["Rev","`Rev` is an iterator that produces elements in reverse order. This struct is created by the `rev()` method on `IndexedParallelIterator`"],["Skip","`Skip` is an iterator that skips over the first `n` elements. This struct is created by the `skip()` method on `IndexedParallelIterator`"],["Split","`Split` is a parallel iterator using arbitrary data and a splitting function. This struct is created by the `split()` function."],["Take","`Take` is an iterator that iterates over the first `n` elements. This struct is created by the `take()` method on `IndexedParallelIterator`"],["TryFold","`TryFold` is an iterator that applies a function over an iterator producing a single value. This struct is created by the `try_fold()` method on `ParallelIterator`"],["TryFoldWith","`TryFoldWith` is an iterator that applies a function over an iterator producing a single value. This struct is created by the `try_fold_with()` method on `ParallelIterator`"],["Update","`Update` is an iterator that mutates the elements of an underlying iterator before they are yielded."],["WhileSome","`WhileSome` is an iterator that yields the `Some` elements of an iterator, halting as soon as any `None` is produced."],["Zip","`Zip` is an iterator that zips up `a` and `b` into a single iterator of pairs. This struct is created by the `zip()` method on `IndexedParallelIterator`"],["ZipEq","An `IndexedParallelIterator` that iterates over two parallel iterators of equal length simultaneously."]],"trait":[["FromParallelIterator","`FromParallelIterator` implements the creation of a collection from a `ParallelIterator`. By implementing `FromParallelIterator` for a given type, you define how it will be created from an iterator."],["IndexedParallelIterator","An iterator that supports \"random access\" to its data, meaning that you can split it at arbitrary indices and draw data from those points."],["IntoParallelIterator","`IntoParallelIterator` implements the conversion to a `ParallelIterator`."],["IntoParallelRefIterator","`IntoParallelRefIterator` implements the conversion to a `ParallelIterator`, providing shared references to the data."],["IntoParallelRefMutIterator","`IntoParallelRefMutIterator` implements the conversion to a `ParallelIterator`, providing mutable references to the data."],["ParallelBridge","Conversion trait to convert an `Iterator` to a `ParallelIterator`."],["ParallelExtend","`ParallelExtend` extends an existing collection with items from a `ParallelIterator`."],["ParallelIterator","Parallel version of the standard iterator trait."]]});