use std::cell::RefCell;
use crate::io::{FileType, Reader, reader_factory, Writer, writer_factory};
use crate::network::Network;
use crate::ui::presenter::Presenter;

pub struct Model {
    reader: RefCell<Box<dyn Reader>>,
    writer: Box<dyn Writer>,
    network: Network
}

impl Model {
    pub fn new(in_file_type: FileType, out_file_type: FileType) -> Self {
        Model {
            reader: RefCell::new(Box::new(reader_factory(in_file_type))),
            writer: Box::new(writer_factory(out_file_type)),
            network: Network::new()
        }
    }

    pub fn assign_workers(&self, infile: String, outfile: String, pres: &Presenter) {
        let read_result = self.reader.borrow_mut().read_file(infile, &self.network);
        if read_result.is_err() {
            pres.report_error(read_result.err().unwrap().to_string());
            return;
        }

        // TODO: pass callback to update progress bar
        let solve_result = self.network.find_min_cost_max_flow();
        if solve_result.is_err() {
            pres.report_error(solve_result.err().unwrap().to_string());
            return;
        }

        let write_result = self.writer.write_file(&self.network, outfile);
        if write_result.is_err() {
            pres.report_error(write_result.err().unwrap().to_string());
            return;
        }

        pres.report_success();
    }
}
