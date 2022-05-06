use std::collections::HashMap;

mod node;
mod arc;

#[test]
fn test_push_flow() {
    // setup
    let node_a_id = 65498;
    let node_b_id = 63524657;
    let mut nodes = HashMap::new();
    nodes.insert(node_a_id,node::Node::new(node_a_id));
    nodes.insert(node_b_id, node::Node::new(node_b_id));
    let mut arc = arc::Arc::new(node_a_id, node_b_id, 16.8,
                                     0, 1, &mut nodes);

    // test
    assert_eq!(nodes.get(&node_a_id).unwrap().get_num_connected_nodes(), 1);
    assert_eq!(nodes.get(&node_b_id).unwrap().get_num_connected_nodes(), 0);
    assert_eq!(nodes.get(&node_a_id).unwrap().get_first_connected_node_id(), Some(node_b_id));
    arc.push_flow(&mut nodes);
    assert_eq!(nodes.get(&node_a_id).unwrap().get_num_connected_nodes(), 0);
    assert_eq!(nodes.get(&node_b_id).unwrap().get_num_connected_nodes(), 1);
    assert_eq!(nodes.get(&node_b_id).unwrap().get_first_connected_node_id(), Some(node_a_id));
    // arc.cost
    // arc.start_node
    // arc.end_node
}
