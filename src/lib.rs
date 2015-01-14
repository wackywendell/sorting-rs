/*! Basic sorting algorithms, just for fun.

Note that the exported `Sortable` and `Sorted` traits provide all necessary functionality. 
`Sortable` provides functions for in-place sorting, and `Sorted` provides copy-sorting.

Some benchmarks, all performed on a `Vec` of 2000 random `uint`s:

```ignore
test benchmarks::bench_bubblesort ... bench:   1197517 ns/iter (+/- 154803)
test benchmarks::bench_heapsort   ... bench:     54756 ns/iter (+/- 1863)
test benchmarks::bench_mergesort  ... bench:    132263 ns/iter (+/- 15198)
test benchmarks::bench_quicksort  ... bench:     44623 ns/iter (+/- 9263)
test benchmarks::bench_selsort    ... bench:    699363 ns/iter (+/- 36305)
test benchmarks::bench_shellsort  ... bench:     59090 ns/iter (+/- 3630)
test benchmarks::bench_slice_sort ... bench:     66509 ns/iter (+/- 3064)
```

`bubblesort` is by far the slowest, followed by `selsort`; both unsurprisingly slow.

Interestingly, `quicksort`, `heapsort`, and possibly `shellsort` outperform 
`std::slice::MutableOrdSlice::sort()`, at least on this benchmark; the `std` one is, as I understand
it, an optimized mergesort, and is probably faster on more general benchmarks. Here are some things
this benchmark doesn't cover:

  * Comparing with a `compare` function instead of comparision by-value
  * Sorting a partially-sorted list
  * Sorting something with expensive copy operations
  * Sorting a reverse-sorted list

*/
#![warn(non_camel_case_types)]
#![warn(non_snake_case)]
#![warn(unused_qualifications)]
#![warn(non_upper_case_globals)]
#![warn(missing_docs)]

pub mod algorithms;

#[cfg(test)]
mod benchmarks;

// Public traits, for export

/// In-place sorting methods
pub trait Sortable<T : Ord> : SliceExt<Item=T> {
	/// Quicksort, in-place
	fn quicksort(&mut self){algorithms::quicksort(self.as_mut_slice())}
	/// heapsort, in-place
	fn heapsort(&mut self){algorithms::heapsort(self.as_mut_slice())}
	/// bubblesort, in-place
	fn bubblesort(&mut self){algorithms::bubblesort(self.as_mut_slice())}
	/// selection sort, in-place
	fn selsort(&mut self) {algorithms::selsort(self.as_mut_slice())}
	/// shell sort, in-place
	fn shellsort(&mut self) {algorithms::shellsort::<algorithms::ShellKnuth, T>(self.as_mut_slice())}
}

/// Copy-and-sort methods (i.e., mergesort)
pub trait Sorted<T : Ord + Clone> : AsSlice<T> {
	/// merge sort, returning a sorted version
	fn mergesorted(&self) -> Vec<T> {algorithms::mergesort(self.as_slice())}
}


impl<T: Ord> Sortable<T> for [T]{
	
}

impl<'a, T: Ord + Clone> Sorted<T> for &'a [T]{}

