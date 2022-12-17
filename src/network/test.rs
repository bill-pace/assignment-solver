use crate::network::Network;
use crate::ui::CurrentStatus;

#[test]
fn test_push_flow() {
    // setup
    let node_a_id = 0;
    let node_b_id = 1;
    let cost = 16.8;
    let network = Network::new();
    network.add_arc(node_a_id, node_b_id, cost, 1, 1);

    // test
    assert_eq!(network.nodes.borrow()[node_a_id].get_num_connections(), 1);
    assert_eq!(network.nodes.borrow()[node_b_id].get_num_connections(), 0);
    assert_eq!(network.arcs.borrow()[network.nodes.borrow()[node_a_id].get_first_connected_arc_id().unwrap()].get_end_node_id(),
               node_b_id);
    network.push_flow_down_path(&vec![0, 1]);
    assert_eq!(network.nodes.borrow()[node_a_id].get_num_connections(), 0);
    assert_eq!(network.nodes.borrow()[node_b_id].get_num_connections(), 1);
    assert_eq!(network.arcs.borrow()[network.nodes.borrow()[node_b_id].get_first_connected_arc_id().unwrap()].get_end_node_id(),
               node_a_id);
    assert_eq!(network.arcs.borrow()[0].get_cost(), -cost);
    assert_eq!(network.arcs.borrow()[0].get_start_node_id(), node_b_id);
    assert_eq!(network.arcs.borrow()[0].get_end_node_id(), node_a_id);
}

#[test]
fn test_shortest_path() {
    // setup
    let network = Network::new();
    let task_names: Vec<String> = vec!["Task 1".into(), "Task 2".into()];
    // add task 1
    network.add_task(task_names[0].clone(), 1, 1);
    network.add_task(task_names[1].clone(),1, 1);
    network.add_worker("Worker 1".into(),
                       &vec![(&task_names[0], 2.5_f32),
                             (&task_names[1], 3.0_f32)]);
    network.add_worker("Worker 2".into(),
                       &vec![(&task_names[0], 2.6_f32),
                             (&task_names[1], 1.9_f32)]);

    // test
    assert_eq!(network.nodes.borrow().len(), 6);
    assert_eq!(network.arcs.borrow().len(), 8);
    let mut path = network.find_shortest_path().unwrap();
    assert_eq!(path.len(), 4);
    assert_eq!(*path.first().unwrap(), 0);
    assert_eq!(*path.last().unwrap(), 1);
    assert_eq!(network.get_path_cost(&path), 1.9_f32);
    network.push_flow_down_path(&path);
    path.reverse();
    for node_pair in path.windows(2) {
        network.find_connecting_arc_id(node_pair[0], node_pair[1])
            .expect(&*format!("Arc between {} and {} not inverted!", node_pair[1], node_pair[0]));
    }
}

#[test]
fn test_min_cost_augmentation() {
    // setup
    let network = Network::new();
    let task_names: Vec<String> = vec!["Task 1".into(), "Task 2".into(), "Task 3".into(),
                          "Task 4".into(), "Task 5".into()];
    let worker_names: Vec<String> = vec![
        "Worker 1".to_string(),
        "Worker 2".to_string(),
        "Worker 3".to_string(),
        "Worker 4".to_string(),
        "Worker 5".to_string(),
        "Worker 6".to_string(),
        "Worker 7".to_string(),
        "Worker 8".to_string(),
        "Worker 9".to_string(),
        "Worker 10".to_string(),
    ];
    network.add_task(task_names[0].clone(), 1, 2);
    network.add_task(task_names[1].clone(), 2, 2);
    network.add_task(task_names[2].clone(), 0, 2);
    network.add_task(task_names[3].clone(), 2, 3);
    network.add_task(task_names[4].clone(), 1, 2);
    network.add_worker(worker_names[0].clone(),
                       &vec![(&task_names[0], 3.0),
                             (&task_names[1], 4.0), (&task_names[2], 1.5),
                             (&task_names[3], 1.5), (&task_names[4], 5.0)]);
    network.add_worker(worker_names[1].clone(),
                       &vec![(&task_names[0], 4.0),
                             (&task_names[1], 3.0), (&task_names[2], 6.0),
                             (&task_names[3], 2.0), (&task_names[4], 1.0)]);
    network.add_worker(worker_names[2].clone(),
                       &vec![(&task_names[0], 2.0),
                             (&task_names[1], 5.0), (&task_names[2], 4.0),
                             (&task_names[3], 1.0), (&task_names[4], 3.0)]);
    network.add_worker(worker_names[3].clone(),
                       &vec![(&task_names[0], 3.0),
                             (&task_names[1], 5.0), (&task_names[2], 1.0),
                             (&task_names[3], 4.0), (&task_names[4], 0.0)]);
    network.add_worker(worker_names[4].clone(),
                       &vec![(&task_names[0], 1.0),
                             (&task_names[1], 4.0), (&task_names[2], 2.0),
                             (&task_names[3], 3.0), (&task_names[4], 5.0)]);
    network.add_worker(worker_names[5].clone(),
                       &vec![(&task_names[0], 5.0),
                             (&task_names[1], 3.0), (&task_names[2], 1.0),
                             (&task_names[3], 4.0), (&task_names[4], 2.0)]);
    network.add_worker(worker_names[6].clone(),
                       &vec![(&task_names[0], 1.0),
                             (&task_names[1], 3.0), (&task_names[2], 5.0),
                             (&task_names[3], 4.0), (&task_names[4], 2.0)]);
    network.add_worker(worker_names[7].clone(),
                       &vec![(&task_names[0], 4.0),
                             (&task_names[1], 3.0), (&task_names[2], 5.0),
                             (&task_names[3], 1.0), (&task_names[4], 2.0)]);
    network.add_worker(worker_names[8].clone(),
                       &vec![(&task_names[0], 5.0),
                             (&task_names[1], 2.0), (&task_names[2], 3.0),
                             (&task_names[3], 4.0), (&task_names[4], 1.0)]);
    network.add_worker(worker_names[9].clone(),
                       &vec![(&task_names[0], 2.0),
                             (&task_names[1], 5.0), (&task_names[2], 1.0),
                             (&task_names[3], 3.0), (&task_names[4], 4.0)]);

    // test
    assert_eq!(network.nodes.borrow().len(), 17);
    assert_eq!(network.arcs.borrow().len(), 65);
    assert_eq!(network.nodes.borrow()[0].get_num_connections(), 10);
    assert_eq!(network.nodes.borrow()[1].get_num_connections(), 1);
    network.find_min_cost_max_flow(&std::sync::Arc::new(CurrentStatus::new())).unwrap();
    let total_cost = -network.get_cost_of_arcs_from_nodes(&task_names);
    assert_eq!(network.nodes.borrow()[0].get_num_connections(), 0);
    assert_eq!(network.nodes.borrow()[1].get_num_connections(), 4);
    assert!((total_cost - 12.5_f32).abs() / 12.5_f32 < 5e-10_f32);
}
