mod node;
mod arc;

#[test]
fn test_push_flow() {
    // setup
    let node_a = node::Node::new(65498);
    let node_b = node::Node::new(1);
    let arc = arc::Arc::new();

    // test
    arc.push_flow();
    assert_eq!(node_a.get_num_connected_nodes(), 0);
    assert_eq!(node_b.get_num_connected_nodes(), 1);
    assert_eq!(node_b.get_first_connected_node_id(), Some(65498_usize));
    // arc.cost
    // arc.start_node
    // arc.end_node
}
