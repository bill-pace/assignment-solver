//! Reader and Writer traits that work directly with Network structs, for simplified use in a user
//! interface module. New Readers and Writers should get submodules to the io module (eg io::csv)
//! and can implement both Reader and Writer on appropriate structs there. A factory can then
//! produce an appropriate Reader and Writer for the chosen file type(s).

use crate::network::Network;

mod csv;

pub(crate) trait Reader {
    fn read_file(filename: &String) -> Network;
}

pub(crate) trait Writer {
    fn write_file(results: &Network);
}

pub(crate) fn reader_factory(file_type: &String) -> Result<impl Reader, std::io::Error> {
    match file_type.as_str() {
        "csv" => CsvReader::new(),
        _ => std::io::Error("No reader implemented for file type {}", file_type)
    }
}

pub(crate) fn writer_factory(file_type: &String) -> Result<impl Writer, std::io::Error> {
    match file_type.as_str() {
        "csv" => CsvWriter::new(),
        _ => std::io::Error("No writer implemented for file type {}", file_type)
    }
}
