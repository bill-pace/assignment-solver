use std::cell::{Cell, RefCell};
use std::sync::Arc;
use crate::io::{FileType, Reader, reader_factory, Writer, writer_factory};
use crate::network::Network;
use crate::ui::{CurrentStatus, Status};
use crate::ui::presenter::Presenter;

pub struct Model {
    reader: RefCell<Option<Box<dyn Reader>>>,
    writer: RefCell<Option<Box<dyn Writer>>>,
    network: Network
}

impl Model {
    pub fn new() -> Self {
        Model {
            reader: RefCell::new(None),
            writer: RefCell::new(None),
            network: Network::new()
        }
    }

    pub fn set_file_types(&self, in_file_type: FileType, out_file_type: FileType) {
        self.reader.replace(Some(Box::new(reader_factory(in_file_type))));
        self.writer.replace(Some(Box::new(writer_factory(out_file_type))));
    }

    pub fn assign_workers(&self, infile: String, outfile: String, status: Arc<CurrentStatus>) {
        let read_result = self.reader.borrow_mut().as_mut().unwrap()
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

        let write_result = self.writer.borrow().as_ref().unwrap()
            .write_file(&self.network, outfile);
        if write_result.is_err() {
            status.set_status(Status::Failure(write_result.unwrap_err().to_string()));
            return;
        }

        status.set_status(Status::Success);
    }
}
