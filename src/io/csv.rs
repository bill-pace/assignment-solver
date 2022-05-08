use std::collections::HashMap;
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
pub struct CsvReader {
    tasks: HashMap<usize, String>,
    workers: HashMap<usize, String>
}

impl CsvReader {
    pub fn new() -> CsvReader {
        CsvReader { tasks: HashMap::new(), workers: HashMap::new() }
    }

    fn process_file<R>(&mut self, mut reader: R) -> std::io::Result<Network>
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
        // while let Some(line) = line_iter.next() {
        //
        // }

        Ok(network)
    }

    fn process_tasks(&mut self, network: &mut Network, task_names: String, task_minima: String,
                     task_maxima: String) -> Result<(), std::io::Error> {
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
            self.tasks.insert(task_id, task_info.0.trim().to_string());
        }
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
        -network.get_cost_of_arcs_from_nodes(&file_reader.tasks.keys()
                                                                     .map(|k| k.clone())
                                                                     .collect());
    assert!((total_cost - 12.5_f32).abs() / 12.5_f32 < 5e-10_f32);
}
