use std::cell::{Cell, RefCell};
use crate::io::{FileType, Reader, reader_factory, Writer, writer_factory};
use crate::network::Network;
use crate::ui::presenter::Presenter;

pub struct Model {
    reader: RefCell<Option<Box<dyn Reader>>>,
    writer: RefCell<Option<Box<dyn Writer>>>,
    network: Network,
    solving: Cell<bool>
}

impl Model {
    pub fn new() -> Self {
        Model {
            reader: RefCell::new(None),
            writer: RefCell::new(None),
            network: Network::new(),
            solving: Cell::new(false)
        }
    }

    pub fn set_file_types(&self, in_file_type: FileType, out_file_type: FileType) {
        self.reader.replace(Some(Box::new(reader_factory(in_file_type))));
        self.writer.replace(Some(Box::new(writer_factory(out_file_type))));
    }

    pub fn assign_workers(&self, infile: String, outfile: String, pres: &Presenter) {
        self.solving.set(true);
        let read_result = self.reader.borrow_mut().as_mut().unwrap()
            .read_file(infile, &self.network);
        if read_result.is_err() {
            pres.report_error(read_result.err().unwrap().to_string());
            self.solving.set(false);
            return;
        }

        // TODO: pass callback to update progress bar
        let solve_result = self.network.find_min_cost_max_flow();
        if solve_result.is_err() {
            pres.report_error(solve_result.err().unwrap().to_string());
            self.solving.set(false);
            return;
        }

        let write_result = self.writer.borrow().as_ref().unwrap()
            .write_file(&self.network, outfile);
        if write_result.is_err() {
            pres.report_error(write_result.err().unwrap().to_string());
            self.solving.set(false);
            return;
        }

        pres.report_success();
        self.solving.set(false);
    }
}
