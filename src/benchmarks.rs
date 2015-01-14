#![cfg(test)]

extern crate test;

use self::test::Bencher;
use std::rand::Rng;

use {Sorted,Sortable};

fn get_bench_vec() -> Vec<u64> {
	let mut rng = ::std::rand::thread_rng();
	rng.gen_iter().take(2000).collect()
}

/// Benchmark of the standard library sort function
#[bench]
fn bench_slice_sort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<u64> = test_vec.clone();
		v.as_mut_slice().sort();
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
		v.as_slice().mergesorted()
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

