//! Reader and Writer traits that work directly with Network structs, for simplified use in a user
//! interface module. New Readers and Writers should get submodules to the io module (eg io::csv)
//! and can implement both Reader and Writer on appropriate structs there. A factory can then
//! produce an appropriate Reader and Writer for the chosen file type(s), based on a chosen item in
//! the FileType enum. The enum should have one entry for every filetype supported by
//! implementations of the Reader and Writer traits.

use crate::io::csv::{CsvReader, CsvWriter};
use crate::network::Network;

mod csv;

pub enum FileType {
    CSV
}

pub(crate) trait Reader {
    fn read_file(&mut self, filename: &str) -> std::io::Result<Network>;
}

pub(crate) trait Writer {
    fn write_file(results: &Network) -> std::io::Result<()>;
}

pub(crate) fn reader_factory(file_type: FileType) -> impl Reader {
    match file_type {
        FileType::CSV => CsvReader::new()
    }
}

pub(crate) fn writer_factory(file_type: FileType) -> impl Writer {
    match file_type {
        FileType::CSV => CsvWriter::new()
    }
}
