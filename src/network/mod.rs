//! # Network
//!
//! This module contains definitions and implementations for the Network struct, as well as
//! submodules for the Node and Arc structs. A network stores its constituent nodes and arcs in
//! HashMaps keyed on the nodes' IDs, allowing the usize IDs to be used for access instead of
//! keeping borrowed references alive longer than strictly necessary.

use std::collections::HashMap;

mod node;
mod arc;

/// A Network is a collection of nodes and the arcs that connect those nodes.
pub(crate) struct Network {
    num_nodes: usize,
    min_flow_satisfied: bool,
    min_flow_amount: usize,
    nodes: Vec<node::Node>,
    arcs: HashMap<(usize, usize), arc::Arc>
}

impl Network {
    /// Create a new Network and its source and sink nodes, ensuring those two nodes have IDs 0 and
    /// 1.
    pub fn new() -> Network {
        let mut new_network = Network { num_nodes: 0, min_flow_satisfied: false, min_flow_amount: 0,
                                        nodes: Vec::new(), arcs: HashMap::new() };
        new_network.add_node();
        new_network.add_node();
        new_network
    }

    /// Add a new node to the network representing a task, and connect that node to the sink. Return
    /// the task's ID number to look it back up after assignment is complete.
    pub fn add_task(&mut self, min_workers: usize, max_workers: usize) -> usize {
        let task_id = self.add_node();

        self.min_flow_amount += min_workers;
        if min_workers > 0 {
            // end node is the sink; cost is 0 because this arc does not connect workers to tasks
            self.add_arc(task_id, 1, 0.0,
                         min_workers, max_workers);
        } else {
            // draw in reverse order as above since this task is already at its minimum requirement
            self.add_arc(1, task_id, 0.0,
                         min_workers, max_workers);
        }

        task_id
    }

    /// Add a new node to the network representing a worker, connect the source to the new node, and
    /// connect the new node to all tasks the worker can perform. As with add_task, return the
    /// worker node's ID.
    pub fn add_worker(&mut self, task_affinity: &Vec<(usize, f32)>) -> usize {
        let worker_id = self.add_node();
        // connect source to worker - no cost here, and each worker can be assigned exactly once so
        // the flow bound is 1 for both phases of the min cost augmentation
        self.add_arc(0, worker_id, 0.0, 1, 1);
        // connect the worker to each task they can perform, using their affinity as the cost of the
        // new arc - flow bound stays 1
        for affinity in task_affinity {
            self.add_arc(worker_id, affinity.0, affinity.1,
                         1, 1);
        }
        worker_id
    }

    /// Perform minimum cost augmentation to build a min cost max flow by assigning one worker at a
    /// time.
    pub fn find_min_cost_max_flow(&mut self) {
        let mut current_flow = 0_usize;
        if self.min_flow_amount == 0 {
            self.reset_arcs_for_second_phase();
        }

        // Connections from the source are unassigned workers - loop until they're all assigned.
        while self.nodes[0].get_num_connected_nodes() > 0 {
            // find shortest path from source to sink - if no path found, then notify the user that
            // the assignment is infeasible
            // TODO: add shortcut based on lowest worker affinity
            let path = self.find_shortest_path();

            // path found, push flow and increment the amount of flow
            self.push_flow_down_path(&path);
            current_flow += 1;
            if current_flow == self.min_flow_amount {
                // minimum requirement achieved: invert arcs that touch the sink
                self.reset_arcs_for_second_phase();
            }
        }
    }

    /// Create a new Node and add it to the network's HashMap of nodes.
    fn add_node(&mut self) -> usize {
        let new_node = node::Node::new();
        self.nodes.push(new_node);
        let node_id = self.num_nodes;
        self.num_nodes += 1;
        node_id
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
        let mut nodes_updated = Vec::new(); // stores ID numbers
        nodes_updated.push(0);
        let mut num_iterations = 0_usize;
        while nodes_updated.len() > 0 && num_iterations < self.num_nodes {
            let nodes_to_search_from = nodes_updated.clone();
            nodes_updated.clear();

            // for each node updated in the last iteration, see if any of its existing connections
            // result in a shorter path to any other node than what's been found so far
            for node_id in &nodes_to_search_from {
                let node = &self.nodes[*node_id];
                for connected_node_id in node.get_connections() {
                    // calculate distances
                    let cur_dist = distances[*connected_node_id];
                    let dist_to_here = distances[*node_id];
                    let dist_from_here =
                        match self.arcs.get(&(*node_id, *connected_node_id)) {
                            Some(arc) => arc.get_cost(),
                            None => {
                                panic!("Could not find arc from {} to {}!",
                                       *node_id, *connected_node_id);
                            }
                        };

                    if dist_to_here + dist_from_here < cur_dist {
                        // found a shorter path to the connected node
                        distances[*connected_node_id] = dist_to_here + dist_from_here;
                        predecessors[*connected_node_id] = Some(node_id.clone());
                        if *connected_node_id != 1 {
                            // omit arcs leaving the sink, as these arcs cannot be part of a path to
                            // the sink (else it would be a walk instead of a path) and their
                            // representation within the code is an imperfect mirror of the residual
                            // network for the sake of keeping their data in memory
                            nodes_updated.push(*connected_node_id);
                        }
                    }
                }
            }

            num_iterations += 1;
            // eliminate duplicated entries to make sure we only search once before an update
            nodes_updated.sort();
            nodes_updated.dedup();
        }

        // if no path to sink found, or number of iterations exceeds number of nodes, there's a bug
        // TODO: no path found may also be an infeasible problem specification from the inputs,
        //       rather than a bug in the code
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
            let arc = self.arcs.get(&(node_pair[0], node_pair[1])).unwrap();
            let arc_inverted = arc.push_flow(self.min_flow_satisfied, &mut self.nodes);
            if arc_inverted {
                let arc = self.arcs.remove(&(node_pair[0], node_pair[1])).unwrap();
                self.arcs.insert((node_pair[1], node_pair[0]), arc);
            }
        }
    }

    /// Get cost of flow from arcs leaving the supplied node(s). If the supplied node IDs are the
    /// task node IDs, this method will return -1 times the total cost of worker assignments, since
    /// assigning a worker to a task involves negating the corresponding arc's cost.
    pub fn get_cost_of_arcs_from_nodes(&self, nodes: &Vec<usize>) -> f32 {
        nodes.iter()
             .map(|node| self.nodes[*node]
                             .get_connections().iter()
                             .map(|connected_node| self.arcs.get(&(*node, *connected_node))
                                                       .unwrap().get_cost())
                             .sum::<f32>())
             .sum()
    }

    /// The second phase of minimum cost augmentation starts with all tasks having their minimum
    /// requirement satisfied, and allows further assignment of all remaining workers up to the max
    /// for each task. This method resets all arcs touching the sink to account for the
    /// corresponding changes in the residual network.
    fn reset_arcs_for_second_phase(&mut self) {
        let connections = self.nodes[1].get_connections().clone();
        for connection in connections {
            let arc = self.arcs.get(&(1, connection)).unwrap();
            let arc_inverted = arc.update_for_second_phase(&mut self.nodes);
            if arc_inverted {
                let arc = self.arcs.remove(&(1, connection)).unwrap();
                self.arcs.insert((connection, 1), arc);
            }
        }
    }
}

#[test]
fn test_push_flow() {
    // setup
    let node_a_id = 0;
    let node_b_id = 1;
    let cost = 16.8;
    let mut nodes = Vec::new();
    nodes.push(node::Node::new());
    nodes.push(node::Node::new());
    let arc = arc::Arc::new(node_a_id, node_b_id, cost,
                                     1, 1, &mut nodes);

    // test
    assert_eq!(nodes[node_a_id].get_num_connected_nodes(), 1);
    assert_eq!(nodes[node_b_id].get_num_connected_nodes(), 0);
    assert_eq!(*nodes[node_a_id].get_first_connected_node_id().unwrap(),
               node_b_id);
    arc.push_flow(false, &mut nodes);
    assert_eq!(nodes[node_a_id].get_num_connected_nodes(), 0);
    assert_eq!(nodes[node_b_id].get_num_connected_nodes(), 1);
    assert_eq!(*nodes[node_b_id].get_first_connected_node_id().unwrap(),
               node_a_id);
    assert_eq!(arc.get_cost(), -cost);
    assert_eq!(arc.get_start_node_id(), node_b_id);
    assert_eq!(arc.get_end_node_id(), node_a_id)
}

#[test]
fn test_shortest_path() {
    // setup
    let mut network = Network::new();
    let mut task_ids = Vec::new();
    // add task 1
    task_ids.push(network.add_task(1, 1));
    task_ids.push(network.add_task(1, 1));
    network.add_worker(&vec![(task_ids[0], 2.5_f32), (task_ids[1], 3.0_f32)]);
    network.add_worker(&vec![(task_ids[0], 2.6_f32), (task_ids[1], 1.9_f32)]);

    // test
    assert_eq!(network.nodes.len(), 6);
    assert_eq!(network.arcs.len(), 8);
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

#[test]
fn test_min_cost_augmentation() {
    // setup
    let mut network = Network::new();
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
    assert_eq!(network.nodes.len(), 17);
    assert_eq!(network.arcs.len(), 65);
    assert_eq!(network.nodes[0].get_num_connected_nodes(), 10);
    network.find_min_cost_max_flow();
    let total_cost = -network.get_cost_of_arcs_from_nodes(&task_ids);
    assert_eq!(network.nodes[0].get_num_connected_nodes(), 0);
    assert!((total_cost - 12.5_f32).abs() / 12.5_f32 < 5e-10_f32);
}
