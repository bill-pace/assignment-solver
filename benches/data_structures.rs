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

benchmark_group!(benches, vector, hash_map);
benchmark_main!(benches);
