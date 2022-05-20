//! Structs that implement the Reader and Writer traits for CSV-formatted files.

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;
use crate::io::{Reader, Writer};
use crate::network::Network;
mod test;

/// A reader for CSV-formatted input data. It will populate its lists of task and worker IDs as it
/// reads the file and passes input from that file into the network struct it helps build.
/// CSV inputs should be structured as follows:
///       <ignored>   |   Task Name 1   |   Task Name 2   |   Task Name 3   | ...
///     --------------|-----------------|-----------------|-----------------|----
///       <ignored>   |   Task 1 Min    |   Task 2 Min    |   Task 3 Min    | ...
///     --------------|-----------------|-----------------|-----------------|----
///       <ignored>   |   Task 1 Max    |   Task 2 Max    |   Task 3 Max    | ...
///     --------------|-----------------|-----------------|-----------------|----
///     Worker 1 Name | Task 1 Affinity | Task 2 Affinity | Task 3 Affinity | ...
///     --------------|-----------------|-----------------|-----------------|----
///     Worker 2 Name | Task 1 Affinity | Task 2 Affinity | Task 3 Affinity | ...
///     --------------|-----------------|-----------------|-----------------|----
///     ...
/// Task minima and maxima must be non-negative integers, and setting the max to 0 will be treated
/// as 0 rather than as infinite. Affinities can be any 32-bit floating-point value, including
/// negative numbers, and if left blank will represent an unacceptable assignment (e.g. the worker
/// cannot do the corresponding task).
pub struct CsvReader {
    // keep list of task IDs to pair up with affinities when reading worker data
    tasks: RefCell<Vec<usize>>
}

impl CsvReader {
    /// Create a new CsvReader struct
    pub fn new() -> CsvReader {
        CsvReader { tasks: RefCell::new(Vec::new()) }
    }

    /// Read a provided file line by line to construct a Network from it
    fn process_file<R>(&mut self, reader: R) -> std::io::Result<Network>
        where R: BufRead {
        let network = Network::new();
        let mut line_iter = reader.lines();

        // initialize tasks
        let task_names = match line_iter.next() {
            Some(line) => line?,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   "Empty input file!"))
        };
        let task_minima = match line_iter.next() {
            Some(line) => line?,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   "No minimum requirements for tasks!"))
        };
        let task_maxima = match line_iter.next() {
            Some(line) => line?,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   "No maximum capacities for tasks!"))
        };
        self.process_tasks(&network, task_names, task_minima, task_maxima)?;

        // initialize workers
        while let Some(line) = line_iter.next() {
            match line {
                Ok(l) => self.process_worker(&network, l)?,
                Err(err) => return Err(err)
            }
        }

        Ok(network)
    }

    /// Construct the tasks from lists of their names and the lower and upper bounds on number of
    /// assigned workers
    fn process_tasks(&mut self, network: &Network, task_names: String, task_minima: String,
                     task_maxima: String) -> std::io::Result<()> {
        let names = task_names.split(",").collect::<Vec<&str>>();
        let minima = task_minima.split(",").collect::<Vec<&str>>();
        let maxima = task_maxima.split(",").collect::<Vec<&str>>();
        if names.len() != minima.len() || names.len() != maxima.len() {
            // mismatched input sizes imply either missing or extra data and thus bad input format
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                           "Mismatched input data for tasks: each task must have both \
                                           a minimum and a maximum number of workers specified."));
        }

        for task_info in zip(names, zip(minima, maxima)).skip(1) {
            let minimum = match usize::from_str(task_info.1.0.trim()) {
                Ok(m) => m,
                Err(err) =>
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   format!(r#"Expected integer minimum, found "{}"; error: {}"#,
                                                           task_info.1.0, err)))
            };
            let maximum = match usize::from_str(task_info.1.1.trim()) {
                Ok(m) => m,
                Err(err) =>
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   format!(r#"Expected integer maximum, found "{}"; error: {}"#,
                                                           task_info.1.1, err)))
            };
            let name = task_info.0.trim().to_string();
            let task_id = network.add_task(name, minimum, maximum);
            self.tasks.borrow_mut().push(task_id);
        }
        Ok(())
    }

    /// Add a new worker to the network under construction
    fn process_worker(&mut self, network: &Network, worker_info: String) -> std::io::Result<()> {
        let mut affinities = Vec::new();
        let mut info = worker_info.split(",");
        let worker_name = info.next()
            .expect("Problem reading worker's name!")
            .trim().to_string();

        for task_id in self.tasks.borrow().iter() {
            let val = match info.next() {
                Some(v) => v,
                None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                       format!("Too few task affinities for worker {}!",
                                                               worker_name)))
            };

            if val != "" {
                let aff = match f32::from_str(val) {
                    Ok(v) => v,
                    Err(err) =>
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                       format!(r#"Expected numeric value for worker affinity, found "{}"; error: {}"#,
                                                               val, err)))
                };
                affinities.push((*task_id, aff)); // task ID stored in self.tasks
            }
        }

        network.add_worker(worker_name, &affinities);

        Ok(())
    }
}

impl Reader for CsvReader {
    fn read_file(&mut self, filename: &str) -> std::io::Result<Network> {
        let f = File::open(filename)?;
        self.process_file(BufReader::new(f))
    }
}

pub struct CsvWriter {

}

impl CsvWriter {
    pub fn new() -> CsvWriter {
        CsvWriter { }
    }
}

impl Writer for CsvWriter {
    fn write_file(results: &Network, filename: &str) -> std::io::Result<()> {
        Ok(())
    }
}
