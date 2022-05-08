use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;
use crate::io::{Reader, Writer};
use crate::network::Network;

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
    tasks: Vec<(usize, String)>,
    workers: Vec<(usize, String)>
}

impl CsvReader {
    pub fn new() -> CsvReader {
        CsvReader { tasks: Vec::new(), workers: Vec::new() }
    }

    fn process_file<R>(&mut self, reader: R) -> std::io::Result<Network>
        where R: BufRead {
        let mut network = Network::new();
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
        self.process_tasks(&mut network, task_names, task_minima, task_maxima)?;

        // initialize workers
        while let Some(line) = line_iter.next() {
            match line {
                Ok(l) => self.process_worker(&mut network, l)?,
                Err(err) => return Err(err)
            }
        }

        Ok(network)
    }

    fn process_tasks(&mut self, network: &mut Network, task_names: String, task_minima: String,
                     task_maxima: String) -> std::io::Result<()> {
        // TODO: handle missing info
        for task_info in zip(task_names.split(","),
                             zip(task_minima.split(","), task_maxima.split(",")))
            .skip(1) {
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
            let task_id = network.add_task(minimum, maximum);
            self.tasks.push((task_id, task_info.0.trim().to_string()));
        }
        Ok(())
    }

    fn process_worker(&mut self, network: &mut Network, worker_info: String)
        -> std::io::Result<()> {
        let mut affinities = Vec::new();
        let mut info = worker_info.split(",");
        let worker_name = info.next().unwrap().trim().to_string();

        let mut task_idx = 0;
        while let Some(val) = info.next() {
            if val != "" {
                let aff = match f32::from_str(val) {
                    Ok(v) => v,
                    Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                        format!(r#"Expected numeric value for worker affinity, found "{}"; error: {}"#, val, err)))
                };
                affinities.push((self.tasks[task_idx].0, aff)); // task ID stored in self.tasks
            }
            task_idx += 1;
        }

        let worker_id = network.add_worker(&affinities);
        self.workers.push((worker_id, worker_name));
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
    fn write_file(results: &Network) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_read() {
    let mut file_reader = CsvReader::new();
    let mut network = file_reader.read_file("src/io/test-data/testInput.csv").unwrap();
    network.find_min_cost_max_flow();
    let total_cost =
        -network.get_cost_of_arcs_from_nodes(&file_reader.tasks.iter()
                                                                     .map(|t| t.0)
                                                                     .collect());
    assert!((total_cost - 12.5_f32).abs() / 12.5_f32 < 5e-10_f32);
}

#[test]
fn test_read_errors() {
    let mut file_reader1 = CsvReader::new();
    let net1 = file_reader1.read_file("src/io/test-data/inputEmpty.csv");
    assert!(net1.is_err());
    let net1_err = net1.err().unwrap();
    assert_eq!(net1_err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(net1_err.to_string(), "Empty input file!");

    let mut file_reader2 = CsvReader::new();
    let net2 = file_reader2.read_file("src/io/test-data/inputBadMin.csv");
    assert!(net2.is_err());
    assert_eq!(net2.err().unwrap().to_string(),
               r#"Expected integer minimum, found "a"; error: invalid digit found in string"#);

    let mut file_reader3 = CsvReader::new();
    let net3 = file_reader3.read_file("src/io/test-data/inputBadMax.csv");
    assert!(net3.is_err());
    assert_eq!(net3.err().unwrap().to_string(),
               r#"Expected integer maximum, found "b"; error: invalid digit found in string"#);

    let mut file_reader4 = CsvReader::new();
    let net4 = file_reader4.read_file("src/io/test-data/inputBadAffinity.csv");
    assert!(net4.is_err());
    assert_eq!(net4.err().unwrap().to_string(),
               r#"Expected numeric value for worker affinity, found "c"; error: invalid float literal"#);
}
