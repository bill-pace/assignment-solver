use std::cell::RefCell;
use std::sync::Arc;
use crate::io::{FileType, Reader, reader_factory, Writer, writer_factory};
use crate::network::Network;
use crate::ui::{CurrentStatus, Status};

pub(super) struct Solver {
    reader: RefCell<Box<dyn Reader>>,
    writer_type: FileType,
    network: Network,
}

impl Solver {
    pub fn new(in_file_type: FileType, out_file_type: FileType) -> Self {
        Solver {
            reader: RefCell::new(Box::new(reader_factory(in_file_type))),
            writer_type: out_file_type,
            network: Network::new()
        }
    }

    pub fn assign_workers(&self, infile: String, outfile: String, status: &Arc<CurrentStatus>) {
        let read_result = self.reader.borrow_mut()
            .read_file(infile, &self.network);
        if let Err(e) = read_result {
            status.set_status(Status::Failure(e.to_string()));
            return;
        }

        let solve_result = self.network.find_min_cost_max_flow(status);
        if let Err(e) = solve_result {
            status.set_status(Status::Failure(e.message));
            return;
        }

        let write_result = writer_factory(self.writer_type,
        self.reader.borrow().clone_task_names())
            .write_file(&self.network, outfile);
        if let Err(e) = write_result {
            status.set_status(Status::Failure(e.to_string()));
            return;
        }

        status.set_status(Status::Success);
    }
}
