#![cfg(test)]

use algorithms::{partition,merge,get_parent,get_leaves,heapify,ShellHs,ShellKnuth};
use algorithms::{quicksort,heapsort,selsort,bubblesort,mergesort,shellsort};

////////////////////////////////////////////////////////////////////////////////////////////////
// Quicksort tests
#[test]
fn test_partition() {
	let tests : &mut [uint] = [1u,2,3];
	let result : &mut [uint] = [1,2,3];
	let p = partition(tests, 1);
	assert_eq!((&tests, p), (&result, 1));
	
	let p = partition(tests, 0);
	assert_eq!((&tests, p), (&result, 0));
	
	let p = partition(tests, 2);
	assert_eq!((&tests, p), (&result, 2));
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 1);
	let result : &mut [uint] = [1,2,3];
	assert_eq!((&tests, p), (&result, 2));
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 0);
	let result : &mut [uint] = [1,3,2];
	assert_eq!((&tests, p), (&result, 0));
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 2);
	let result : &mut [uint] = [1,2,3];
	assert_eq!((&tests, p), (&result, 1));
	
	let tests : &mut [uint] = [1u,4,5,3,2];
	let p = partition(tests, 2);
	let result : &mut [uint] = [1,4,2,3,5];
	assert_eq!((&tests, p), (&result, 4));
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

fn get_test_vecs() -> Vec<Vec<uint>> {
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
		let test_slice = test_vec.as_mut_slice();
		println!("Unsorted: {}", test_slice);
		quicksort(test_slice);
		println!("Sorted:   {}", test_slice);
		assert!(is_sorted(test_slice));
	}
}


////////////////////////////////////////////////////////////////////////////////////////////////
// Mergesort Tests

#[test]
fn test_merge(){
	let (test_slice1, test_slice2) : (&[uint], &[uint]) = ([], []);
	assert_eq!(merge(test_slice1, test_slice2), vec!());
	
	let test_slice3 = [1,2,4,5];
	assert_eq!(merge(test_slice1, test_slice3), vec!(1,2,4,5));
	assert_eq!(merge(test_slice3, test_slice1), vec!(1,2,4,5));
	assert_eq!(merge(test_slice3, test_slice3), vec!(1,1,2,2,4,4,5,5));
	
	let test_slice4 = [3];
	assert_eq!(merge(test_slice3, test_slice4), vec!(1,2,3,4,5));
	assert_eq!(merge(test_slice4, test_slice3), vec!(1,2,3,4,5));
}



#[test]
fn test_mergesort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		let v = mergesort(test_slice);
		assert!(is_sorted(v.as_slice()));
	}
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

	for i in range(0, 21){
		let (l, r) = get_leaves(i);
		assert_eq!(get_parent(l), i);
		assert_eq!(get_parent(r), i);
	}
}

fn is_max_heap<T : Ord>(slice : &[T]) -> bool{
	for i in range(1, slice.len()){
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
		let test_slice = test_vec.as_mut_slice();
		heapify(test_slice);
		println!("Heapifying: {} -> {}", unsorted_vec.as_slice(), test_slice)
		assert!(is_max_heap(test_slice));
	}
}

#[test]
fn test_heapsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		heapsort(test_slice);
		assert!(is_sorted(test_slice));
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Selection sort Tests

#[test]
fn test_selectionsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		selsort(test_slice);
		assert!(is_sorted(test_slice));
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Bubblesort Tests

#[test]
fn test_bubblesort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		bubblesort(test_slice);
		assert!(is_sorted(test_slice));
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////
// Shellsort Tests
#[test]
fn test_shell_hs_knuth() {
	let hs : Vec<uint> = ShellHs::new(363).collect();
	assert_eq!(hs, vec!(121, 40, 13, 4, 1));
	let hs : Vec<uint> = ShellHs::new(362).collect();
	assert_eq!(hs, vec!(40, 13, 4, 1));
	let hs : Vec<uint> = ShellHs::new(2).collect();
	assert_eq!(hs, vec!(1));
}

#[test]
fn test_shellsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		shellsort::<ShellKnuth, uint>(test_slice);
		assert!(is_sorted(test_slice));
	}
}
