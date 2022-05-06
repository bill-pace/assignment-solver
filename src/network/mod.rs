mod node;
mod arc;

#[test]
fn test_push_flow() {
    // setup
    let node_a_id = 65498_usize;
    let node_a = node::Node::new(node_a_id);
    let node_b = node::Node::new(1);
    let mut arc = arc::Arc::new(node_a_id, 1, 16.8, 0, 1);

    // test
    arc.push_flow();
    assert_eq!(node_a.get_num_connected_nodes(), 0);
    assert_eq!(node_b.get_num_connected_nodes(), 1);
    assert_eq!(node_b.get_first_connected_node_id(), Some(node_a_id));
    // arc.cost
    // arc.start_node
    // arc.end_node
}
