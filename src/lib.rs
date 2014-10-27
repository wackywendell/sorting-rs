/// Utility functions for my scripts.

#[warn(non_camel_case_types)]
#[warn(unnecessary_qualification)]
#[warn(non_uppercase_statics)]
#[warn(missing_doc)]

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

pub use sorting::quicksort;

pub mod sorting;

/// Count the number of occurrences of each value in an iterator
pub fn counter<K : std::hash::Hash + Eq, I : Iterator<K>>(mut list : I) -> HashMap<K, uint> {
	let mut counter : HashMap<K, uint> = HashMap::new();
	for key in list {
		match counter.entry(key) {
			Vacant(entry) => {entry.set(1);},
			Occupied(entry) => {(*entry.into_mut()) += 1;}
		}
	}
	counter
}

#[test]
fn test_counter() {
	let my_list : Vec<&str> = vec!();
	let count : HashMap<&str, uint> = counter(my_list.into_iter());
	assert_eq!(count.find(&"a"), None);
	
	let my_list = vec!("a", "b", "cd", "a", "a", "b");
	let count : HashMap<&str, uint> = counter(my_list.into_iter());
	
	assert_eq!(count.find(&"a"), Some(&3u));
	assert_eq!(count.find(&"b"), Some(&2u));
	assert_eq!(count.find(&"cd"), Some(&1u));
	assert_eq!(count.find(&"e"), None);
}
