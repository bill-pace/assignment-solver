mod node;
mod arc;

#[test]
fn test_push_flow() {
    // setup
    let node_a = node::Node::new();
    let node_b = node::Node::new();
    let arc = arc::Arc::new();

    // test
    arc.push_flow();
    assert_eq!(node_a.connected_nodes.len(), 0);
    assert_eq!(node_b.connected_nodes.len(), 1);
    assert_eq!(node_b.connected_nodes[0], node_a.id);
    // arc.cost
    // arc.start_node
    // arc.end_node
}
