#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::collections::HashMap;
use std::cell::RefCell;

fn vector(bench: &mut Bencher) {
    let mut distances = vec![f32::INFINITY; 5102];
    bench.iter(|| {
        distances[2345] = 4.0;
    })
}

fn hash_map(bench: &mut Bencher) {
    let mut distances: HashMap<usize, f32> = HashMap::new();
    for i in 0..5101 {
        distances.insert(i, f32::INFINITY);
    }

    bench.iter(|| {
        distances.insert(2345, 4.0);
    })
}

fn read_vector(bench: &mut Bencher) {
    let mut distances = vec![f32::INFINITY; 5102];
    bench.iter(|| {
        let i = distances[2345];
    })
}

fn read_hash_map(bench: &mut Bencher) {
    let mut distances: HashMap<usize, f32> = HashMap::new();
    for i in 0..5101 {
        distances.insert(i, f32::INFINITY);
    }

    bench.iter(|| {
        let i = distances.get(&2345).unwrap();
    })
}

fn clone_vec(bench: &mut Bencher) {
    let v = RefCell::new(vec![0_usize, 5000]);

    bench.iter(|| {
        let cv = v.borrow().clone();
    })
}

fn borrow_vec(bench: &mut Bencher) {
    let v = RefCell::new(vec![0_usize, 5000]);

    bench.iter(|| {
        let rv = v.borrow();
    })
}

benchmark_group!(write_benches, vector, hash_map);
benchmark_group!(read_benches, read_vector, read_hash_map);
benchmark_group!(ref_vector, clone_vec, borrow_vec);
benchmark_main!(write_benches, read_benches, ref_vector);
