use std::cell::RefCell;
use std::sync::Arc;
use crate::io::{FileType, Reader, reader_factory, Writer, writer_factory};
use crate::network::Network;
use crate::ui::{CurrentStatus, Status};

pub struct Model {
    reader: RefCell<Box<dyn Reader>>,
    writer: RefCell<Box<dyn Writer>>,
    network: Network
}

impl Model {
    pub fn new(in_file_type: FileType, out_file_type: FileType) -> Self {
        Model {
            reader: RefCell::new(Box::new(reader_factory(in_file_type))),
            writer: RefCell::new(Box::new(writer_factory(out_file_type))),
            network: Network::new()
        }
    }

    pub fn assign_workers(&self, infile: String, outfile: String, status: Arc<CurrentStatus>) {
        let read_result = self.reader.borrow_mut()
            .read_file(infile, &self.network);
        if read_result.is_err() {
            status.set_status(Status::Failure(read_result.unwrap_err().to_string()));
            return;
        }

        let solve_result = self.network.find_min_cost_max_flow(status.clone());
        if solve_result.is_err() {
            status.set_status(Status::Failure(solve_result.unwrap_err().message));
            return;
        }

        let write_result = self.writer.borrow()
            .write_file(&self.network, outfile);
        if write_result.is_err() {
            status.set_status(Status::Failure(write_result.unwrap_err().to_string()));
            return;
        }

        status.set_status(Status::Success);
    }
}
