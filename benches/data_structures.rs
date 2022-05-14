#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::collections::HashMap;

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

benchmark_group!(write_benches, vector, hash_map);
benchmark_group!(read_benches, read_vector, read_hash_map);
benchmark_main!(write_benches, read_benches);
