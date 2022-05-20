use std::env;
use crate::io::{FileType, Reader, Writer, reader_factory, writer_factory};

mod network;
mod io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} INPUT_FILENAME OUTPUT_FILENAME", args[0]);

        std::process::exit(1);
    }

    let mut reader = reader_factory(FileType::CSV);
    let network = reader.read_file(args[1].to_string()).unwrap();
    network.find_min_cost_max_flow().unwrap();
    let writer = writer_factory(FileType::CSV);
    writer.write_file(&network, args[2].to_string()).unwrap();
}
