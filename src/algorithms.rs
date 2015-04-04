//! Algorithms that back up the Sortable and Sorted traits

use std;
fn choose_pivot<T : Ord>(slice : &[T]) -> usize {
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
pub fn partition<T : Ord>(slice : &mut [T], pivot : usize) -> usize {
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
	let right_slice = &mut right_slice[1..];
	
	quicksort(left_slice);
	quicksort(right_slice);
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Heapsort

/// Index of parent node
#[inline]
pub fn get_parent(ix : usize) -> usize {
	(ix+1) / 2 - 1
}

/// Indices of leaf nodes
#[inline]
pub fn get_leaves(ix : usize) -> (usize, usize) {
	(ix*2 + 1, ix*2+2)
}

/// Turn the array into a maximal heap
pub fn heapify<T : Ord>(slice : &mut [T]){
	for ix in 1..slice.len(){
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
	for i in 0..(ln - 1){
		//let portion = slice.slice_to_mut(ln - i);
		let portion = &mut slice[..(ln - i)];
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
	
	merge(v1.as_ref(), v2.as_ref())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Selection sort

/// The selection sort algorithm.
pub fn selsort<T : Ord>(slice : &mut [T]){
	if slice.len() < 2 {return}

	let mut min = 0;
	for i in 1..slice.len(){
		if slice[i] < slice[min] {
			min = i;
		}
	}
	slice.swap(0, min);

	selsort(&mut slice[1..]);
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Bubblesort

/// The bubblesort algorithm.
pub fn bubblesort<T : Ord>(slice : &mut [T]){
	for n in ((slice.len() as isize)..1isize).step_by(-1isize){
		for m in 1..(n as usize){
			if slice[m] < slice[m-1] {slice.swap(m, m-1);}
		}
	}
}


////////////////////////////////////////////////////////////////////////////////////////////////
// Shell sort

/// The values to go by for a shell-sort. Note that the sequence determines the complexity.
pub trait ShellHs : Iterator<Item=usize> {
	/// Create a new ShellHs, for a vector of length n
	fn new(n: usize) -> Self;
}

/// Knuth's values: 1,4,13,40,121... up to n/3
#[derive(Copy, Clone, Debug)]
pub struct ShellKnuth {
	h : usize
}

impl Iterator for ShellKnuth {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		self.h /= 3;
		match self.h {
			0 => None,
			value => Some(value)
		}
	}
}

impl ShellHs for ShellKnuth {
	fn new(n: usize) -> ShellKnuth {
		let mut h = 4;
		while h*3 <= n {
			h = 3*h + 1;
		}
		
		ShellKnuth{h: h}
	}
}

fn insertion_sort_partial<T : Ord>(slice : &mut [T], start: usize, step: usize){
	for i in ((start+step)..slice.len()).step_by(step) {
		let mut curloc = i;
		while (curloc >= step) && slice[curloc] < slice[curloc-step] {
			slice.swap(curloc, curloc-step);
			curloc -= step;
		}
	}
}

/// Shell sort
pub fn shellsort<H : ShellHs, T : Ord>(slice : &mut [T]){
	let hs : H = ShellHs::new(slice.len());
	for h in hs {
		for k in (0..h) {
			// our sublist is now [k, h+k, 2h+k,...]
			// We insertion sort it
			insertion_sort_partial(slice, k, h);
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
#[test]
fn test_partition() {
	let tests : &mut [u64] = &mut [1,2,3];
	let result : &mut [u64] = &mut [1,2,3];
	let p = partition(tests, 1);
	assert_eq!((&tests, p), (&result, 1));
	
	let p = partition(tests, 0);
	assert_eq!((&tests, p), (&result, 0));
	
	let p = partition(tests, 2);
	assert_eq!((&tests, p), (&result, 2));
	
	let tests : &mut [u64] = &mut [1,3,2];
	let p = partition(tests, 1);
	let result : &mut [u64] = &mut [1,2,3];
	assert_eq!((&tests, p), (&result, 2));
	
	let tests : &mut [u64] = &mut [1,3,2];
	let p = partition(tests, 0);
	let result : &mut [u64] = &mut [1,3,2];
	assert_eq!((&tests, p), (&result, 0));
	
	let tests : &mut [u64] = &mut [1,3,2];
	let p = partition(tests, 2);
	let result : &mut [u64] = &mut [1,2,3];
	assert_eq!((&tests, p), (&result, 1));
	
	let tests : &mut [u64] = &mut [1,4,5,3,2];
	let p = partition(tests, 2);
	let result : &mut [u64] = &mut [1,4,2,3,5];
	assert_eq!((&tests, p), (&result, 4));
}


////////////////////////////////////////////////////////////////////////////////////////////////
// Mergesort Tests

#[test]
fn test_merge(){
	let (test_slice1, test_slice2) : (&[u64], &[u64]) = (&[], &[]);
	assert_eq!(merge(test_slice1, test_slice2), vec!());
	
	let test_slice3 = &[1,2,4,5];
	assert_eq!(merge(test_slice1, test_slice3), vec!(1,2,4,5));
	assert_eq!(merge(test_slice3, test_slice1), vec!(1,2,4,5));
	assert_eq!(merge(test_slice3, test_slice3), vec!(1,1,2,2,4,4,5,5));
	
	let test_slice4 = &[3];
	assert_eq!(merge(test_slice3, test_slice4), vec!(1,2,3,4,5));
	assert_eq!(merge(test_slice4, test_slice3), vec!(1,2,3,4,5));
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Heapsort Tests

#[test]
fn test_indexing(){
	assert_eq!(get_parent(1), 0);
	assert_eq!(get_parent(2), 0);
	assert_eq!(get_parent(3), 1);
	assert_eq!(get_parent(4), 1);
	assert_eq!(get_parent(5), 2);
	assert_eq!(get_parent(6), 2);
	assert_eq!(get_parent(7), 3);

	for i in 0usize..21usize {
		let (l, r) = get_leaves(i);
		assert_eq!(get_parent(l), i);
		assert_eq!(get_parent(r), i);
	}
}

#[cfg(test)]
fn is_max_heap<T : Ord>(slice : &[T]) -> bool{
	for i in (1..slice.len()){
		let p = get_parent(i);
		if slice[p] < slice[i] {return false;}
	}
	return true;
}

#[test]
fn test_heapify(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let unsorted_vec = test_vec.clone();
		let test_slice = test_vec.as_mut();
		heapify(test_slice);
		println!("Heapifying: {:?} -> {:?}", unsorted_vec, test_slice);
		assert!(is_max_heap(test_slice));
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Shellsort Tests
#[test]
fn test_shell_hs_knuth() {
	let shell : ShellKnuth = ShellHs::new(363);
	let hs : Vec<usize> = shell.collect();
	assert_eq!(hs, vec!(121, 40, 13, 4, 1));
	let shell : ShellKnuth = ShellHs::new(362);
	let hs : Vec<usize> = shell.collect();
	assert_eq!(hs, vec!(40, 13, 4, 1));
	let shell : ShellKnuth = ShellHs::new(2);
	let hs : Vec<usize> = shell.collect();
	assert_eq!(hs, vec!(1));
}

/// Test if a slice is in a sorted state.
pub fn is_sorted<T : Ord>(slice: &[T]) -> bool {
	for win in slice.windows(2){
		match win {
			[ref a, ref b] if a <= b => continue,
			[_, _] => return false,
			_ => panic!("slice.windows(2) returned a window with size {} != 2", win.len())
		}
	}
	true
}

#[cfg(test)]
fn get_test_vecs() -> Vec<Vec<u64>> {
	vec!(
		vec!(), vec!(1), vec!(1,2), vec!(2,1), vec!(1,2,3), vec!(2,1,3), vec!(3,1,2),
		vec!(8,5,2,6,9,3), vec!(2,3,5,6,8,9), vec!(9,8,6,5,3,2), vec!(8,4,7,3,6,2,5,1),
		vec!(8,1,7,2,6,3,5,4), vec!(8,1,7,2,6,3,5,4),
		vec!(16,14,1,1,7,18,7,6,8,18,5),
		vec!(19,18,14,15,3,9,8,2,2,20,11),
		vec!(2,3,8,7,23,26,19,29,23,32,20,18,11,11,24,13,17),
		vec!(0,3,7,6),
		vec!(6,4,4,5,11,10,10),
		vec!(15,13,17,1,1,19,3,19,0,11),
		vec!(19,19,21,21,22,25,19,14,23,25,14,10,8,4,28,12,2,33),
		vec!(8,1,0,5,3),
		vec!(27,14,22,10,8,23,7,32,28,31,9,19,30,28,21,20,13),
		vec!(2,1,4,17,5,17,8,2,13,13)
	)
}

#[test]
fn test_quicksort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut();
		println!("Unsorted: {:?}", test_slice);
		quicksort(test_slice);
		println!("Sorted:   {:?}", test_slice);
		assert!(is_sorted(test_slice));
	}
}

#[test]
fn test_mergesort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut();
		let v = mergesort(test_slice);
		assert!(is_sorted(v.as_ref()));
	}
}

#[test]
fn test_selectionsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut();
		selsort(test_slice);
		assert!(is_sorted(test_slice));
	}
}

#[test]
fn test_bubblesort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut();
		bubblesort(test_slice);
		assert!(is_sorted(test_slice));
	}
}

#[test]
fn test_shellsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut();
		shellsort::<ShellKnuth, u64>(test_slice);
		assert!(is_sorted(test_slice));
	}
}

#[test]
fn test_heapsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut();
		heapsort(test_slice);
		assert!(is_sorted(test_slice));
	}
}
