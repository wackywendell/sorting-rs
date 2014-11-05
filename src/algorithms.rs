//! Algorithms that back up the Sortable and Sorted traits

use std;
fn choose_pivot<T : Ord>(slice : &[T]) -> uint {
	// if slice.len() <= 2 {return slice.len() - 1;};
	let (mut ismall, imid, mut ibig) = (0, slice.len() / 2, slice.len() - 1);
	if slice[ibig] < slice[ismall] {std::mem::swap(&mut ibig, &mut ismall);}
	if slice[imid] <= slice[ismall] {ismall}
	else if slice[ibig] <= slice[imid] {ibig}
	else{imid}
}

/// choose a pivot, then reorder so that everything to the left of the pivot is smaller, and 
/// everything to the right is greater
/// Assumes slice.len() > 2
pub fn partition<T : Ord>(slice : &mut [T], pivot : uint) -> uint {
	let mxix = slice.len() - 1;
	slice.swap(pivot, mxix);
	let (mut left, mut right) = (0, mxix-1);
	
	while left < right {
		if slice[left] <= slice[mxix] {left += 1;}
		else if slice[right] >= slice[mxix] {right -= 1;}
		else {
			slice.swap(left, right); 
			left += 1;
			right -= 1;
		}
	}
	
	if left > right {
		// We just swapped the final two.
		slice.swap(left, mxix);
		return left;
	}
	
	// Left and right met.
	if slice[left] >= slice[mxix] {
		slice.swap(left, mxix);
		return left;
	} else if slice[left] <= slice[mxix] {
		slice.swap(left+1, mxix);
		return left+1;
	}
	
	panic!("This should be unreachable. Indices: {}, {} / {}", left, right, mxix);
}

/// The quicksort algorithm
pub fn quicksort<T : Ord>(slice : &mut [T]){
	if slice.len() <= 1 {return;}
	else if slice.len() == 2 {
		if slice[0] >= slice[1] {slice.swap(0,1);}
		return;
	}
	
	let pivot = choose_pivot(slice);
	let pivot = partition(slice, pivot);
	let (left_slice, right_slice) = slice.split_at_mut(pivot);
	// left_slice is [0 - pivot-1], right_slice is [pivot, end]. We don't want to include the
	// pivot, so reassign right_slice
	let right_slice = right_slice.tail_mut();
	
	quicksort(left_slice);
	quicksort(right_slice);
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Heapsort

/// Index of parent node
#[inline]
pub fn get_parent(ix : uint) -> uint {
	(ix+1) / 2 - 1
}

/// Indices of leaf nodes
#[inline]
pub fn get_leaves(ix : uint) -> (uint, uint) {
	(ix*2 + 1, ix*2+2)
}

/// Turn the array into a maximal heap
pub fn heapify<T : Ord>(slice : &mut [T]){
	for ix in range(1, slice.len()){
		let mut curix = ix;
		while curix > 0 {
			let pix = get_parent(curix);
			if slice[pix] > slice[curix] {break;}
			
			slice.swap(pix, curix);
			curix = pix;
		}
	}
}

/// Assuming our slice is a heap, take the maximal element (element 0), swap it to the end,
/// take that end-element / now root and filter it down the heap until its in the right place.
/// At the end of this function, the max element is at the end, and elements 0 to (end-1) are a heap
/// again.
fn heap_pop<T : Ord>(slice : &mut [T]){
	if slice.len() <= 1 {return;}
	let mxix = slice.len() - 2; // last index in the new heap
	slice.swap(0, mxix+1);
	
	// Now we filter downwards.
	let mut curix = 0;
	loop {
		let (l,r) = get_leaves(curix);
		if l > mxix {
			// we reached the bottom, there are no more leaves.
			break;
		}
		let switch_ix = if (r > mxix) || (slice[l] > slice[r]) {l} else {r};
		if slice[curix] >= slice[switch_ix] {break;}
		slice.swap(curix, switch_ix);
		curix = switch_ix;
	}
}

/// Turn a heap-array into a sorted array
pub fn heap_to_sorted<T : Ord>(slice : &mut [T]){
	//~ let mut portion = slice;
	//~ while portion.len() > 1 {
		//~ heap_pop(portion);
		//~ portion = portion.init_mut();
	//~ }
	
	let ln = slice.len();
	if ln <= 1 {return;}
	for i in range(0, ln - 1){
		let portion = slice.slice_to_mut(ln - i);
		heap_pop(portion);
	}
}

/// The heapsort algorithm.
/// This turns the array into an in-place binary max heap, then uses that to sort the list.
pub fn heapsort<T : Ord>(slice : &mut [T]){
	heapify(slice);
	heap_to_sorted(slice);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Merge Sort

/// Merge two sorted arrays into a single vector
pub fn merge<T : Ord + Clone>(slice1 : &[T], slice2 : &[T]) -> Vec<T> {
	let mut vec = Vec::with_capacity(slice1.len() + slice2.len());
	
	let (mut it1, mut it2) = (slice1.iter().peekable(), slice2.iter().peekable());
	
	loop {
		let push_v = match (it1.peek(), it2.peek()) {
			(None, None) => break,
			(Some(&v), None) => {it1.next(); v.clone()},
			(Some(&v1), Some(&v2)) if v1 <= v2 => {it1.next(); v1.clone()},
			(_, Some(&v)) => {it2.next(); v.clone()}
		};
		vec.push(push_v);
	}
	return vec;
}

/// Basic mergesort. NOT in-place
pub fn mergesort<T : Ord + Clone>(slice : &[T]) -> Vec<T> {
	match slice {
		[] => {return vec!();},
		[ref v] => {return vec!(v.clone());},
		_ => {}
	}
	let (s1, s2) = slice.split_at(slice.len() / 2);
	let v1 = mergesort(s1);
	let v2 = mergesort(s2);
	
	merge(v1.as_slice(), v2.as_slice())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Selection sort

/// The selection sort algorithm.
pub fn selsort<T : Ord>(slice : &mut [T]){
	if slice.len() < 2 {return}

	let mut min = 0;
	for i in range(1, slice.len()){
		if slice[i] < slice[min] {
			min = i;
		}
	}
	slice.swap(0, min);

	selsort(slice.slice_from_mut(1));
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Bubblesort

/// The bubblesort algorithm.
pub fn bubblesort<T : Ord>(slice : &mut [T]){
	for n in std::iter::range_step(slice.len() as int, 1i, -1i){	
		for m in range(1, n as uint){
			if slice[m] < slice[m-1] {slice.swap(m, m-1);}
		}
	}
}


////////////////////////////////////////////////////////////////////////////////////////////////
// Shell sort

/// The values to go by for a shell-sort. Note that the sequence determines the complexity.
pub trait ShellHs : Iterator<uint>{
	/// Create a new ShellHs, for a vector of length n
	fn new(n: uint) -> Self;
}

/// Knuth's values: 1,4,13,40,121... up to n/3
pub struct ShellKnuth {
	h : uint
}

impl Iterator<uint> for ShellKnuth {
	fn next(&mut self) -> Option<uint>{
		self.h /= 3;
		match self.h {
			0 => None,
			value => Some(value)
		}
	}
}

impl ShellHs for ShellKnuth {
	fn new(n: uint) -> ShellKnuth {
		let mut h = 4;
		while h*3 <= n {
			h = 3*h + 1;
		}
		
		ShellKnuth{h: h}
	}
}

fn insertion_sort_partial<T : Ord>(slice : &mut [T], start: uint, step: uint){
	for i in std::iter::range_step(start+step, slice.len(), step) {
		let mut curloc = i;
		while (curloc >= step) && slice[curloc] < slice[curloc-step] {
			slice.swap(curloc, curloc-step);
			curloc -= step;
		}
	}
}

/// Shell sort
pub fn shellsort<H : ShellHs, T : Ord>(slice : &mut [T]){
	let mut hs : H = ShellHs::new(slice.len());
	for h in hs {
		for k in range(0,h) {
			// our sublist is now [k, h+k, 2h+k,...]
			// We insertion sort it
			insertion_sort_partial(slice, k, h);
		}
	}
}

