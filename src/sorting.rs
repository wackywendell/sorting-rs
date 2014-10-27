/// Basic sorting algorithms.

#[warn(non_camel_case_types)]
#[warn(unnecessary_qualification)]
#[warn(non_uppercase_statics)]
#[warn(missing_doc)]

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
fn partition<T : Ord>(slice : &mut [T], pivot : uint) -> uint {
	let mut pivot = pivot;
	let mxix = slice.len() - 1;
	let (mut left, mut right) = (0, mxix);
	while left + 1 < right {
		let mut retry = false;
		if slice[left] < slice[pivot] {left += 1; retry = true;}
		if slice[right] > slice[pivot] {right -= 1; retry = true;}
		if left == pivot {left += 1; retry == true;}
		if right == pivot {right -= 1; retry == true;}
		if retry {continue;}
		
		slice.swap(left, right);
	}
	
	if pivot < left && pivot < right && slice[left] < slice[pivot] {slice.swap(left, pivot); pivot = left;}
	else if pivot > right && pivot > left && slice[right] > slice[pivot] {slice.swap(right, pivot); pivot = right;};
	
	return pivot;
}

/// The quicksort algorithm, for sorting an array.
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

#[test]
fn test_partition() {
	let tests : &mut [uint] = [1u,2,3];
	let result : &mut [uint] = [1,2,3];
	let p = partition(tests, 1);
	assert_eq!(&tests, &result);
	assert_eq!(p, 1);
	let p = partition(tests, 0);
	assert_eq!(&tests, &result);
	assert_eq!(p, 0);
	let p = partition(tests, 2);
	assert_eq!(&tests, &result);
	assert_eq!(p, 2);
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 1);
	let result : &mut [uint] = [1,2,3];
	assert_eq!(&tests, &result);
	assert_eq!(p, 2);
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 0);
	let result : &mut [uint] = [1,3,2];
	assert_eq!(&tests, &result);
	assert_eq!(p, 0);
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 2);
	let result : &mut [uint] = [1,2,3];
	assert_eq!(&tests, &result);
	assert_eq!(p, 1);
}

#[test]
fn test_quicksort(){
	fn is_sorted<T : Ord>(slice: &[T]) -> bool {
		for win in slice.windows(2){
			match win {
				[ref a, ref b] if a < b => continue,
				[_, _] => return false,
				_ => fail!("What's going on with windows?")
			}
		}
		true
	}
	
	let test_slice : &mut [uint] = [];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [1];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [1, 2];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [2,1];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [1,2,3];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [2,1,3];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [3,1,2];
	quicksort(test_slice);
	assert!(is_sorted(test_slice));
	
	let test_slice : &mut [uint] = [8,5,2,6,9,3];
	let sorted : &mut [uint] = [2,3,5,6,8,9];
	quicksort(test_slice);
	assert_eq!(&test_slice, &sorted);
	
	let test_slice : &mut [uint] = [2,3,5,6,8,9];
	quicksort(test_slice);
	assert_eq!(&test_slice, &sorted);
	
	let test_slice : &mut [uint] = [9,8,6,5,3,2];
	quicksort(test_slice);
	assert_eq!(&test_slice, &sorted);
}

