//! Structs that implement the Reader and Writer traits for CSV-formatted files.

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::iter::zip;
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
                                               format!("Maximum cannot be less than minimum!")));
            }

            let task_name = name.trim().to_string();
            let task_id = network.add_task(task_name, lower, upper);
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
    /// Create file handle and pass it to the process_file method for reading
    fn read_file(&mut self, filename: String) -> std::io::Result<Network> {
        let f = File::open(filename)?;
        self.process_file(BufReader::new(f))
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
pub struct CsvWriter {

}

impl CsvWriter {
    /// Create a new CsvWriter
    pub fn new() -> CsvWriter {
        CsvWriter { }
    }

    /// Write outputs collected from a Network into a file handle, in CSV format
    fn write(&self, outputs: &Network, mut file: File) -> std::io::Result<()> {
        // freeze order of tasks for writing lines
        let task_ids = outputs.get_task_ids();

        // record final "score" of solution - sum of affinity scores over assignments that were made
        // note that affinity scores are negated as a result of the assignment happening, so we need
        // to negate the total score
        writeln!(file, "Total score:,{}", -outputs.get_cost_of_arcs_from_nodes(&task_ids))?;

        // record task names
        let task_names = outputs.get_task_names(&task_ids);
        writeln!(file, "{}", task_names.join(","))?;

        // create vector of strings that shows worker assignments for each task
        let assignments = self.get_assignments(&task_ids, &outputs);

        // write each line of workers assigned
        for assignment in assignments {
            writeln!(file, "{}", assignment)?;
        }

        Ok(())
    }

    /// Create a vector of comma-delimited strings from the worker-task assignments in a network
    fn get_assignments(&self, task_order: &Vec<usize>, outputs: &Network) -> Vec<String> {
        let worker_assignments = outputs.get_worker_assignments();
        let max_size = worker_assignments.values()
            .map(|v| v.len())
            .max().unwrap();
        let mut assignments: Vec<Vec<String>> = vec![vec![]; max_size];
        for task in task_order {
            for (row, worker) in worker_assignments.get(task).unwrap().iter().enumerate() {
                assignments[row].push(outputs.get_worker_name_from_id(*worker))
            }
            if worker_assignments.get(task).unwrap().len() < max_size {
                for row in worker_assignments.get(task).unwrap().len()..max_size {
                    assignments[row].push("".to_string());
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
