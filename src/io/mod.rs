//! #IO
//!
//! This module defines Reader and Writer traits that work directly with Network structs, for
//! simplified use in a user interface module. New Readers and Writers should be submodules to the
//! io module (eg io::csv) and can implement both Reader and Writer on appropriate structs there. A
//! factory can then produce an appropriate Reader and Writer for the chosen file type(s), based on
//! a chosen item in the FileType enum. The enum should have one entry for every filetype supported
//! by implementations of the Reader and Writer traits.

use crate::io::csv::{CsvReader, CsvWriter};
use crate::network::Network;

mod csv;

/// Supported file types
pub enum FileType {
    CSV
}

/// A Reader will attempt to construct a Network from an input file, returning a Result that
/// indicates whether it had any issues parsing the input file or, if not, a Network struct.
pub(crate) trait Reader {
    fn read_file(&mut self, filename: &str) -> std::io::Result<Network>;
}

/// A Writer takes a Network struct, extracts its worker-task assignments, and attempts to write the
/// assignments to an output file, returning a Result that indicates whether it was successful.
pub(crate) trait Writer {
    fn write_file(&self, results: &Network, filename: &str) -> std::io::Result<()>;
}

/// Create a struct that implements the Reader trait based on the selected file type from the
/// FileType enum
pub(crate) fn reader_factory(file_type: FileType) -> impl Reader {
    match file_type {
        FileType::CSV => CsvReader::new()
    }
}

/// Create a struct that implements the Writer trait based on the selected file type from the
/// FileType enum
pub(crate) fn writer_factory(file_type: FileType) -> impl Writer {
    match file_type {
        FileType::CSV => CsvWriter::new()
    }
}
