//! Reader and Writer traits that work directly with Network structs, for simplified use in a user
//! interface module. New Readers and Writers should get submodules to the io module (eg io::csv)
//! and can implement both Reader and Writer on appropriate structs there. A factory can then
//! produce an appropriate Reader and Writer for the chosen file type(s).

use crate::io::csv::{CsvReader, CsvWriter};
use crate::network::Network;

mod csv;

pub(crate) trait Reader {
    fn read_file(&mut self, filename: &str) -> std::io::Result<Network>;
}

pub(crate) trait Writer {
    fn write_file(results: &Network) -> std::io::Result<()>;
}

// TODO: factory functions
// pub(crate) fn reader_factory(file_type: &String) -> Result<impl Reader, std::io::Error> {
//     match file_type.as_str() {
//         "csv" => Ok(CsvReader::new()),
//         _ => Err(std::io::Error::new(format!("No reader implemented for file type {}", file_type)))
//     }
// }
//
// pub(crate) fn writer_factory(file_type: &String) -> Result<impl Writer, std::io::Error> {
//     match file_type.as_str() {
//         "csv" => Ok(CsvWriter::new()),
//         _ => Err(std::io::Error::new(format!("No writer implemented for file type {}", file_type)))
//     }
// }
