#![cfg(test)]

#![feature(test)]

extern crate test;
extern crate rand;
extern crate sorting;

use test::Bencher;
use rand::Rng;

use sorting::{Sorted,Sortable};

fn get_bench_vec() -> Vec<u64> {
	let mut rng = rand::thread_rng();
	rng.gen_iter().take(2000).collect()
}

/// Benchmark of the standard library sort function
#[bench]
fn bench_slice_sort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.as_mut().sort();
	});
}

#[bench]
fn bench_quicksort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.quicksort();
	});
}

#[bench]
fn bench_heapsort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.heapsort();
	});
}

#[bench]
fn bench_selsort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.selsort();
	});
}

#[bench]
fn bench_shellsort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.shellsort();
	});
}

#[bench]
fn bench_mergesort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let v : Vec<u64> = test_vec.clone();
		let v_ref : &[u64] = v.as_ref();
		v_ref.mergesorted()
	});
}

#[bench]
fn bench_bubblesort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.bubblesort()
	});
}
