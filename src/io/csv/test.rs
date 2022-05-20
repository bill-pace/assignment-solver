use crate::io::csv::*;

#[test]
fn test_read() {
    let mut file_reader = CsvReader::new();
    let network = file_reader.read_file("src/io/csv/test-data/testInput.csv").unwrap();
    network.find_min_cost_max_flow().unwrap();
    let total_cost = -network.get_cost_of_arcs_from_nodes(&file_reader.tasks.borrow());
    assert!((total_cost - 12.5_f32).abs() / 12.5_f32 < 5e-10_f32);
}

#[test]
fn test_read_empty_input() {
    let mut file_reader = CsvReader::new();
    let net = file_reader.read_file("src/io/csv/test-data/inputEmpty.csv");
    assert!(net.is_err());
    let net_err = net.err().unwrap();
    assert_eq!(net_err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(net_err.to_string(), "Empty input file!");
}

#[test]
fn test_read_bad_task_min() {
    let mut file_reader = CsvReader::new();
    let net = file_reader.read_file("src/io/csv/test-data/inputBadMin.csv");
    assert!(net.is_err());
    assert_eq!(net.err().unwrap().to_string(),
               r#"Expected integer minimum, found "a"; error: invalid digit found in string"#);
}

#[test]
fn test_read_bad_task_max() {
    let mut file_reader = CsvReader::new();
    let net = file_reader.read_file("src/io/csv/test-data/inputBadMax.csv");
    assert!(net.is_err());
    assert_eq!(net.err().unwrap().to_string(),
               r#"Expected integer maximum, found "b"; error: invalid digit found in string"#);
}

#[test]
fn test_read_bad_worker_affinity() {
    let mut file_reader = CsvReader::new();
    let net = file_reader.read_file("src/io/csv/test-data/inputBadAffinity.csv");
    assert!(net.is_err());
    assert_eq!(net.err().unwrap().to_string(),
               r#"Expected numeric value for worker affinity, found "c"; error: invalid float literal"#);
}

#[test]
fn test_read_wrong_number_of_task_data() {
    let mut file_reader = CsvReader::new();
    let net = file_reader.read_file("src/io/csv/test-data/inputExtraData.csv");
    assert!(net.is_err());
    assert_eq!(net.err().unwrap().to_string(),
               "Mismatched input data for tasks: each task must have both a minimum and a maximum \
                number of workers specified.");
}

#[test]
fn test_read_wrong_number_of_affinities() {
    let mut file_reader = CsvReader::new();
    let net = file_reader.read_file("src/io/csv/test-data/inputExtraAffinity.csv");
    assert!(net.is_err());
    assert_eq!(net.err().unwrap().to_string(),
               "Too few task affinities for worker Gina!");
}

#[test]
fn test_write() {
    let mut file_reader = CsvReader::new();
    let network = file_reader.read_file("src/io/csv/test-data/testInput.csv").unwrap();
    network.find_min_cost_max_flow().unwrap();
    let file_writer = CsvWriter::new();
    file_writer.write_file(&network, "src/io/csv/test-output/testOutput.csv").unwrap();
}
