//! Structs that implement the Reader and Writer traits for CSV-formatted files.

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::iter::zip;
use std::rc::Rc;
use std::str::FromStr;
use crate::io::{Reader, Writer};
use crate::network::Network;
#[cfg(test)]
mod test;

/// A reader for CSV-formatted input data. It will populate its lists of task and worker IDs as it
/// reads the file and passes input from that file into the network struct it helps build.
/// CSV inputs should be structured as follows:
///     --------------|-----------------|-----------------|-----------------|----
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
pub(super) struct CsvReader {
    // keep list of task IDs to pair up with affinities when reading worker data
    tasks: RefCell<Vec<Rc<String>>>,
}

impl CsvReader {
    /// Create a new `CsvReader` struct
    pub fn new() -> CsvReader {
        CsvReader { tasks: RefCell::new(Vec::new()) }
    }

    /// Read a provided file line by line to construct a Network from it
    fn process_file<R>(&mut self, reader: R, network: &Network) -> std::io::Result<()>
        where R: BufRead {
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
        self.process_tasks(network, &task_names, &task_minima, &task_maxima)?;

        // initialize workers
        for line in line_iter {
            match line {
                Ok(l) => self.process_worker(network, &l)?,
                Err(err) => return Err(err)
            }
        }

        Ok(())
    }

    /// Construct the tasks from lists of their names and the lower and upper bounds on number of
    /// assigned workers
    fn process_tasks(&mut self, network: &Network, task_names: &str, task_minima: &str,
                     task_maxima: &str) -> std::io::Result<()> {
        let names = task_names.split(',').collect::<Vec<&str>>();
        let minima = task_minima.split(',').collect::<Vec<&str>>();
        let maxima = task_maxima.split(',').collect::<Vec<&str>>();
        if names.len() != minima.len() || names.len() != maxima.len() {
            // mismatched input sizes imply either missing or extra data and thus bad input format
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                           "Mismatched input data for tasks: each task must have both \
                                           a minimum and a maximum number of workers specified."));
        }

        for (name, (minimum, maximum)) in zip(names, zip(minima, maxima)).skip(1) {
            let lower = match usize::from_str(minimum.trim()) {
                Ok(m) => m,
                Err(err) =>
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   format!(r#"Expected integer minimum, found "{}"; error: {}"#,
                                                           minimum, err)))
            };
            let upper = match usize::from_str(maximum.trim()) {
                Ok(m) => m,
                Err(err) =>
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                   format!(r#"Expected integer maximum, found "{}"; error: {}"#,
                                                           maximum, err)))
            };
            if upper < lower {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                               "Maximum cannot be less than minimum!".to_string()));
            }

            let task_name = Rc::new(name.trim().to_string());
            self.tasks.borrow_mut().push(Rc::clone(&task_name));
            network.add_task(task_name, lower, upper);
        }
        Ok(())
    }

    /// Add a new worker to the network under construction
    fn process_worker(&mut self, network: &Network, worker_info: &str) -> std::io::Result<()> {
        let mut affinities = Vec::new();
        let mut info = worker_info.split(',');
        let worker_name = info.next()
            .expect("Problem reading worker's name!")
            .trim().to_string();

        let tasks = self.tasks.borrow();
        for task_name in tasks.iter() {
            let val = match info.next() {
                Some(v) => v,
                None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                       format!("Too few task affinities for worker {}!",
                                                               worker_name)))
            };

            if !val.is_empty() {
                let aff = match f32::from_str(val) {
                    Ok(v) => v,
                    Err(err) =>
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                       format!(r#"Expected numeric value for worker affinity, found "{}"; error: {}"#,
                                                               val, err)))
                };
                affinities.push((task_name, aff)); // task ID stored in self.tasks
            }
        }

        network.add_worker(Rc::new(worker_name), &affinities);

        Ok(())
    }
}

impl Reader for CsvReader {
    /// Create file handle and pass it to the `process_file` method for reading
    fn read_file(&mut self, filename: String, network: &Network) -> std::io::Result<()> {
        let f = File::open(filename)?;
        self.process_file(BufReader::new(f), network)
    }

    fn clone_task_names(&self) -> Vec<Rc<String>> {
        self.tasks.borrow().clone()
    }
}

/// A writer for CSV-formatted output data. Given a network that contains its min cost max flow,
/// a CSV writer will construct and write out a table that looks like this:
///     ----------------|-----------------|-----------------|-----------------|----
///       Total score:  |      <f32>      |                 |                 |----
///     ----------------|-----------------|-----------------|-----------------|----
///       Task Name 1   |   Task Name 2   |   Task Name 3   |   Task Name 4   | ...
///     ----------------|-----------------|-----------------|-----------------|----
///      Worker 1 Name  |  Worker 3 Name  |  Worker 5 Name  |  Worker 7 Name  | ...
///     ----------------|-----------------|-----------------|-----------------|----
///      Worker 2 Name  |  Worker 4 Name  |  Worker 6 Name  |  Worker 8 Name  | ...
///     ----------------|-----------------|-----------------|-----------------|----
///     ...
pub(super) struct CsvWriter {
    task_names: Vec<Rc<String>>,
}

impl CsvWriter {
    /// Create a new `CsvWriter`
    pub fn new(task_names: Vec<Rc<String>>) -> CsvWriter {
        CsvWriter {
            task_names,
        }
    }

    /// Write outputs collected from a Network into a file handle, in CSV format
    fn write(&self, outputs: &Network, mut file: File) -> std::io::Result<()> {
        // record final "score" of solution - sum of affinity scores over assignments that were made
        // note that affinity scores are negated as a result of the assignment happening, so we need
        // to negate the total score
        writeln!(file, "Total score:,{}",
                 -outputs.get_cost_of_arcs_from_nodes(&self.task_names))?;

        // record task names
        writeln!(file, "{}",
                 self.task_names.iter()
                     .map(|tn| String::clone(tn))
                     .collect::<Vec<String>>()
                     .join(","))?;

        // create vector of strings that shows worker assignments for each task
        let assignments = self.get_assignments(outputs);

        // write each line of workers assigned
        for assignment in assignments {
            writeln!(file, "{}", assignment)?;
        }

        Ok(())
    }

    /// Create a vector of comma-delimited strings from the worker-task assignments in a network
    fn get_assignments(&self, outputs: &Network) -> Vec<String> {
        let worker_assignments = outputs.get_worker_assignments();
        let max_size = worker_assignments.values()
            .map(Vec::len)
            .max().unwrap();
        let mut assignments: Vec<Vec<String>> = vec![vec![]; max_size];
        for task in &self.task_names {
            for (row, worker) in worker_assignments
                .get(task).unwrap()
                .iter().enumerate() {
                assignments[row].push(String::clone(&worker));
            }
            if worker_assignments.get(task).unwrap().len() < max_size {
                for empty_assignment in assignments.iter_mut()
                    .take(max_size)
                    .skip(worker_assignments.get(task).unwrap().len()) {
                    empty_assignment.push("".to_string());
                }
            }
        }

        assignments.iter()
            .map(|v| v.join(","))
            .collect()
    }
}

impl Writer for CsvWriter {
    /// Create new file or overwrite existing file, and pass handle to the write method
    fn write_file(&self, results: &Network, filename: String) -> std::io::Result<()> {
        let outfile = OpenOptions::new().write(true).create(true).open(filename)?;
        self.write(results, outfile)?;

        Ok(())
    }
}
