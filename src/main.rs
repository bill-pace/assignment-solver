use crate::io::{FileType, Reader, reader_factory};

mod network;
mod io;

fn main() {
    println!("Hello, world!");
}

#[test]
#[ignore]
fn benchmark() { // complete in 4:13 on a 3.8GHz processor
    puffin::set_scopes_on(true);
    let mut reader = reader_factory(FileType::CSV);
    let network = reader.read_file("src/benchmarkInput.csv").unwrap();
    network.find_min_cost_max_flow().unwrap();
}
