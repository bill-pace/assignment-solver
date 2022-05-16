use crate::io::{FileType, Reader, reader_factory};

mod network;
mod io;

fn main() {
    println!("Hello, world!");
}

#[test]
#[ignore]
fn benchmark() { // complete in 4:13 on a 3.8GHz processor
    let mut reader = reader_factory(FileType::CSV);
    let network = reader.read_file("src/benchmarkInput.csv").unwrap();

    let server_addr = format!("{}:{}",
                              local_ip_address::local_ip().unwrap().to_string(),
                              puffin_http::DEFAULT_PORT);
    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    puffin::set_scopes_on(true);

    network.find_min_cost_max_flow().unwrap();
}
