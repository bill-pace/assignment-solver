//! # Network
//!
//! This module contains definitions and implementations for the Network struct, as well as
//! submodules for the Node and Arc structs. A network stores its constituent nodes and arcs in
//! HashMaps keyed on the nodes' IDs,

use std::collections::HashMap;

mod node;
mod arc;

/// A Network is a collection of nodes and the arcs that connect those nodes.
struct Network {
    num_nodes: usize,
    min_flow_satisfied: bool,
    nodes: HashMap<usize, node::Node>,
    arcs: HashMap<(usize, usize), arc::Arc>
}

impl Network {
    /// Create a new Network and its source and sink nodes, ensuring those two nodes have IDs 0 and
    /// 1.
    fn new() -> Network {
        let mut new_network = Network { num_nodes: 0, min_flow_satisfied: false,
                                        nodes: HashMap::new(), arcs: HashMap::new() };
        new_network.add_node();
        new_network.add_node();
        new_network
    }

    /// Create a new Node and add it to the network's HashMap of nodes.
    fn add_node(&mut self) {
        let new_node = node::Node::new();
        self.nodes.insert(self.num_nodes, new_node);
        self.num_nodes += 1;
    }

    /// Create a new Arc and add it to the network's HashMap of arcs
    fn add_arc(&mut self, start_node_id: usize, end_node_id: usize, cost: f32, min_flow: usize,
               max_flow: usize) {
        let new_arc = arc::Arc::new(start_node_id, end_node_id, cost, min_flow,
                                         max_flow, &mut self.nodes);
        self.arcs.insert((start_node_id, end_node_id), new_arc);
    }

    /// Find the shortest path from the network's source node to its sink node, using an adaptation
    /// of the Bellman-Ford algorithm.
    fn find_shortest_path(&self) -> Vec<usize> {
        // Initialize vectors that represent the paths found so far - at start, we have found no
        // paths, so no node has a found predecessor and all nodes are considered infinite distance
        // from the source, except for the source itself. Node IDs are sequential usize that start
        // from zero to enable using them as indices in these vectors.
        let mut distances = vec![f32::INFINITY; self.num_nodes];
        distances[0] = 0.0;
        let mut predecessors: Vec<Option<usize>> = vec![None; self.num_nodes];

        // Search for shortest path, starting from the source.
        let mut nodes_updated = vec![0]; // stores ID numbers
        let mut num_iterations = 0_usize;
        while nodes_updated.len() > 0 && num_iterations < self.num_nodes {
            let nodes_to_search_from = nodes_updated.clone();
            nodes_updated.clear();

            // for each node updated in the last iteration, see if any of its existing connections
            // result in a shorter path to any other node than what's been found so far
            for node_id in &nodes_to_search_from {
                let node = self.nodes.get(&node_id).unwrap();
                for connected_node_id in node.get_connections() {
                    // calculate distances
                    let cur_dist = distances[*connected_node_id];
                    let dist_to_here = distances[*node_id];
                    let dist_from_here =
                        self.arcs.get(&(*node_id, *connected_node_id)).unwrap().get_cost();

                    if dist_to_here + dist_from_here < cur_dist {
                        // found a shorter path to the connected node
                        distances[*connected_node_id] = dist_to_here + dist_from_here;
                        predecessors[*connected_node_id] = Some(node_id.clone());
                        nodes_updated.push(connected_node_id.clone());
                    }
                }
            }

            num_iterations += 1;
        }

        // if no path to sink found, or number of iterations exceeds number of nodes, there's a bug
        predecessors[1].expect("No path found to sink node!");
        if num_iterations >= self.num_nodes {
            panic!("Negative cycle detected - this can't happen in the algorithm this code \
                   attempts to implement, so there must be a bug.");
        }

        // construct path backwards
        let mut path = vec![1];
        while let Some(node_id) = predecessors[*path.last().unwrap()] {
            path.push(node_id);
        }

        // confirm the last node found was the source
        if !(*path.last().unwrap() == 0) {
            panic!("Path does not start at source!")
        }

        path.reverse();
        path
    }

    /// Get total distance of a path by adding the costs of each arc in the path.
    fn get_path_cost(&self, path: &Vec<usize>) -> f32 {
        path.windows(2)
            .map(|node_pair|
                 self.arcs.get(&(node_pair[0], node_pair[1])).unwrap().get_cost())
            .sum()
    }

    /// Push flow down each arc in a path.
    fn push_flow_down_path(&mut self, path: &Vec<usize>) {
        for node_pair in path.windows(2) {
            let mut arc = self.arcs.remove(&(node_pair[0], node_pair[1])).unwrap();
            arc.push_flow(self.min_flow_satisfied, &mut self.nodes);
            self.arcs.insert((node_pair[1], node_pair[0]), arc);
        }
    }
}

#[test]
fn test_push_flow() {
    // setup
    let node_a_id = 65498;
    let node_b_id = 63524657;
    let cost = 16.8;
    let mut nodes = HashMap::new();
    nodes.insert(node_a_id,node::Node::new());
    nodes.insert(node_b_id, node::Node::new());
    let mut arc = arc::Arc::new(node_a_id, node_b_id, cost,
                                     1, 1, &mut nodes);

    // test
    assert_eq!(nodes.get(&node_a_id).unwrap().get_num_connected_nodes(), 1);
    assert_eq!(nodes.get(&node_b_id).unwrap().get_num_connected_nodes(), 0);
    assert_eq!(nodes.get(&node_a_id).unwrap().get_first_connected_node_id(), Some(node_b_id));
    arc.push_flow(false, &mut nodes);
    assert_eq!(nodes.get(&node_a_id).unwrap().get_num_connected_nodes(), 0);
    assert_eq!(nodes.get(&node_b_id).unwrap().get_num_connected_nodes(), 1);
    assert_eq!(nodes.get(&node_b_id).unwrap().get_first_connected_node_id(), Some(node_a_id));
    assert_eq!(arc.get_cost(), -cost);
    assert_eq!(arc.get_start_node_id(), node_b_id);
    assert_eq!(arc.get_end_node_id(), node_a_id)
}

#[test]
fn test_shortest_path() {
    // setup
    let mut network = Network::new();
    // add task 1
    network.add_node();
    network.add_arc(2, 1, 0.0, 1, 1);
    // add task 2
    network.add_node();
    network.add_arc(3, 1, 0.0, 1, 1);
    // add worker 1
    network.add_node();
    network.add_arc(0, 4, 0.0, 1, 1);
    network.add_arc(4, 2, 2.5, 1, 1);
    network.add_arc(4, 3, 3.0, 1, 1);
    // add worker 2
    network.add_node();
    network.add_arc(0, 5, 0.0, 1, 1);
    network.add_arc(5, 2, 2.6, 1, 1);
    network.add_arc(5, 3, 1.9, 1, 1);

    // test
    let mut path = network.find_shortest_path();
    assert_eq!(path.len(), 4);
    assert_eq!(*path.first().unwrap(), 0);
    assert_eq!(*path.last().unwrap(), 1);
    assert_eq!(network.get_path_cost(&path), 1.9_f32);
    network.push_flow_down_path(&path);
    path.reverse();
    for node_pair in path.windows(2) {
        network.arcs.get(&(node_pair[0], node_pair[1]))
            .expect("Inverted arc not found in network!");
    }
}
