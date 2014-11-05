#![cfg(test)]

extern crate test;

use self::test::Bencher;
use std::rand::Rng;

use {Sorted,Sortable};

fn get_bench_vec() -> Vec<uint> {
	let mut rng = ::std::rand::task_rng();
	Vec::from_fn(2000, |_| {rng.gen()})
}

/// Benchmark of the standard library sort function
#[bench]
fn bench_slice_sort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<uint> = test_vec.clone();
		v.as_mut_slice().sort();
	});
}

#[bench]
fn bench_quicksort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<uint> = test_vec.clone();
		v.quicksort();
	});
}

#[bench]
fn bench_heapsort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<uint> = test_vec.clone();
		v.heapsort();
	});
}

#[bench]
fn bench_selsort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<uint> = test_vec.clone();
		v.selsort();
	});
}

#[bench]
fn bench_shellsort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<uint> = test_vec.clone();
		v.shellsort();
	});
}

#[bench]
fn bench_mergesort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let v : Vec<uint> = test_vec.clone();
		v.mergesorted()
	});
}

#[bench]
fn bench_bubblesort(b : &mut Bencher) {
	let test_vec = get_bench_vec();
	
	b.iter(|| {
		let mut v : Vec<uint> = test_vec.clone();
		v.bubblesort()
	});
}

