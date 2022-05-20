use crate::network::Network;

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
    let mut task_ids = Vec::new();
    // add task 1
    task_ids.push(network.add_task(1, 1));
    task_ids.push(network.add_task(1, 1));
    network.add_worker(&vec![(task_ids[0], 2.5_f32), (task_ids[1], 3.0_f32)]);
    network.add_worker(&vec![(task_ids[0], 2.6_f32), (task_ids[1], 1.9_f32)]);

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
    let mut task_ids = Vec::new();
    let mut worker_ids = Vec::new();
    task_ids.push(network.add_task(1, 2));
    task_ids.push(network.add_task(2, 2));
    task_ids.push(network.add_task(0, 2));
    task_ids.push(network.add_task(2, 3));
    task_ids.push(network.add_task(1, 2));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 3.0),
                                             (task_ids[1], 4.0),
                                             (task_ids[2], 1.5),
                                             (task_ids[3], 1.5),
                                             (task_ids[4], 5.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 4.0),
                                             (task_ids[1], 3.0),
                                             (task_ids[2], 6.0),
                                             (task_ids[3], 2.0),
                                             (task_ids[4], 1.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 2.0),
                                             (task_ids[1], 5.0),
                                             (task_ids[2], 4.0),
                                             (task_ids[3], 1.0),
                                             (task_ids[4], 3.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 3.0),
                                             (task_ids[1], 5.0),
                                             (task_ids[2], 1.0),
                                             (task_ids[3], 4.0),
                                             (task_ids[4], 0.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 1.0),
                                             (task_ids[1], 4.0),
                                             (task_ids[2], 2.0),
                                             (task_ids[3], 3.0),
                                             (task_ids[4], 5.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 5.0),
                                             (task_ids[1], 3.0),
                                             (task_ids[2], 1.0),
                                             (task_ids[3], 4.0),
                                             (task_ids[4], 2.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 1.0),
                                             (task_ids[1], 3.0),
                                             (task_ids[2], 5.0),
                                             (task_ids[3], 4.0),
                                             (task_ids[4], 2.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 4.0),
                                             (task_ids[1], 3.0),
                                             (task_ids[2], 5.0),
                                             (task_ids[3], 1.0),
                                             (task_ids[4], 2.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 5.0),
                                             (task_ids[1], 2.0),
                                             (task_ids[2], 3.0),
                                             (task_ids[3], 4.0),
                                             (task_ids[4], 1.0)]));
    worker_ids.push(network.add_worker(&vec![(task_ids[0], 2.0),
                                             (task_ids[1], 5.0),
                                             (task_ids[2], 1.0),
                                             (task_ids[3], 3.0),
                                             (task_ids[4], 4.0)]));

    // test
    assert_eq!(network.nodes.borrow().len(), 17);
    assert_eq!(network.arcs.borrow().len(), 65);
    assert_eq!(network.nodes.borrow()[0].get_num_connections(), 10);
    network.find_min_cost_max_flow().unwrap();
    let total_cost = -network.get_cost_of_arcs_from_nodes(&task_ids);
    assert_eq!(network.nodes.borrow()[0].get_num_connections(), 0);
    assert!((total_cost - 12.5_f32).abs() / 12.5_f32 < 5e-10_f32);
}
