//! # Network
//!
//! This module contains definitions and implementations for the Network struct, as well as
//! submodules for the Node and Arc structs and a custom error type to represent infeasibility in
//! the problem specification. A network stores its constituent nodes and arcs in vectors and passes
//! their indices to anything that needs to hold a reference to them.

mod node;
mod arc;
mod feasibility_error;
mod test;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use crate::network::feasibility_error::FeasibilityError;

/// A Network is a collection of nodes and the arcs that connect those nodes.
pub(crate) struct Network {
    min_flow_satisfied: Cell<bool>,
    min_flow_amount: Cell<usize>,
    max_flow_amount: Cell<usize>,
    num_tasks: Cell<usize>,
    nodes: RefCell<Vec<node::Node>>,
    arcs: RefCell<Vec<arc::Arc>>,
    task_names: RefCell<HashMap<usize, String>>,
    worker_names: RefCell<HashMap<usize, String>>
}

impl Network {
    /// Create a new Network and its source and sink nodes, ensuring those two nodes have IDs 0 and
    /// 1.
    pub fn new() -> Network {
        let new_network = Network {
            min_flow_satisfied: Cell::new(false),
            min_flow_amount: Cell::new(0),
            max_flow_amount: Cell::new(0),
            num_tasks: Cell::new(0),
            nodes: RefCell::new(Vec::new()),
            arcs: RefCell::new(Vec::new()),
            task_names: RefCell::new(HashMap::new()),
            worker_names: RefCell::new(HashMap::new())
        };
        new_network.add_node(); // flow source, id 0
        new_network.add_node(); // flow sink, id 1
        new_network
    }

    /// Add a new node to the network representing a task, and connect that node to the sink. Return
    /// the task's ID number to look it back up after assignment is complete.
    pub fn add_task(&self, name: String, min_workers: usize, max_workers: usize) -> usize {
        let task_id = self.add_node();

        self.min_flow_amount.set(self.min_flow_amount.get() + min_workers);
        self.max_flow_amount.set(self.max_flow_amount.get() + max_workers);
        self.num_tasks.set(self.num_tasks.get() + 1);
        if min_workers > 0 {
            // end node is the sink; cost is 0 because this arc does not connect workers to tasks
            self.add_arc(task_id, 1, 0.0,
                         min_workers, max_workers);
        } else {
            // draw in reverse order as above since this task is already at its minimum requirement
            self.add_arc(1, task_id, 0.0,
                         min_workers, max_workers);
        }
        self.task_names.borrow_mut().insert(task_id, name);

        task_id
    }

    /// Add a new node to the network representing a worker, connect the source to the new node, and
    /// connect the new node to all tasks the worker can perform. As with add_task, return the
    /// worker node's ID.
    pub fn add_worker(&self, name: String, task_affinity: &Vec<(usize, f32)>) {
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
        self.worker_names.borrow_mut().insert(worker_id, name);
    }

    /// Perform minimum cost augmentation to build a min cost max flow by assigning one worker at a
    /// time.
    pub fn find_min_cost_max_flow(&self) -> Result<(), FeasibilityError> {
        #[cfg(feature = "profiling")] {
            puffin::profile_function!();
        }

        // initial checks for feasibility: make sure number of workers is within the range specified
        // by total min and total max
        let nodes = self.nodes.borrow();
        let num_workers = nodes.len() - self.num_tasks.get() - 2; // 2 are source and sink
        if num_workers < self.min_flow_amount.get() {
            return Err(FeasibilityError { message: "Not enough workers to assign!".to_string() });
        }
        if num_workers > self.max_flow_amount.get() {
            return Err(FeasibilityError {
                message: "Not enough capacity for workers!".to_string()
            });
        }

        let mut current_flow = 0_usize;
        if self.min_flow_amount.get() == 0 {
            self.reset_arcs_for_second_phase();
        }

        // Connections from the source are unassigned workers - loop until they're all assigned.
        while nodes[0].get_num_connections() > 0 {
            // find shortest path from source to sink - if no path found, then notify the user that
            // the assignment is infeasible
            // TODO: add shortcut based on lowest worker affinity
            let path = self.find_shortest_path()?;

            // path found, push flow and increment the amount of flow
            self.push_flow_down_path(&path);
            current_flow += 1;
            if current_flow == self.min_flow_amount.get() {
                // minimum requirement achieved: invert arcs that touch the sink
                self.reset_arcs_for_second_phase();
            }

            #[cfg(feature = "profiling")]
            {
                if current_flow % 100 == 0 {
                    puffin::GlobalProfiler::lock().new_frame();
                }
            }
        }

        Ok(())
    }

    /// Get the keys to the task_names HashMap
    pub fn get_task_ids(&self) -> Vec<usize> {
        self.task_names.borrow()
            .keys()
            .map(|k| *k)
            .collect::<Vec<usize>>()
            .clone()
    }

    /// Get names of tasks in order of requested IDs
    pub fn get_task_names(&self, ids: &Vec<usize>) -> Vec<String> {
        let mut names: Vec<String> = vec!();
        for id in ids {
            names.push(self.task_names.borrow().get(id).unwrap().to_string());
        }
        names
    }

    /// Get cost of flow from arcs leaving the supplied node(s). If the supplied node IDs are the
    /// task node IDs, this method will return -1 times the total cost of worker assignments, since
    /// assigning a worker to a task involves negating the corresponding arc's cost.
    pub fn get_cost_of_arcs_from_nodes(&self, nodes: &Vec<usize>) -> f32 {
        nodes.iter()
            .flat_map(|node|
                self.nodes.borrow()[*node]
                    .get_connections()
                    .iter()
                    .map(|connected_node|
                        self.arcs.borrow()[*connected_node].get_cost())
                    .collect::<Vec<f32>>())
            .sum()
    }

    /// Get name of worker given an ID number
    pub fn get_worker_name_from_id(&self, id: usize) -> String {
        self.worker_names.borrow().get(&id).unwrap().to_string()
    }

    /// Create and return a HashMap of which workers are assigned to which tasks
    pub fn get_worker_assignments(&self) -> HashMap<usize, Vec<usize>> {
        let mut assignments = HashMap::new();
        let task_ids = self.get_task_ids();
        for task in task_ids {
            let workers = self.nodes.borrow()[task].get_connections()
                .iter()
                .map(|a| self.arcs.borrow()[*a].get_end_node_id())
                .filter(|n| *n != 1)
                .collect();
            assignments.insert(task, workers);
        }

        assignments
    }

    /// Create a new Node and add it to the network's collection of nodes.
    fn add_node(&self) -> usize {
        let new_node = node::Node::new();
        let mut nodes = self.nodes.borrow_mut();
        let node_id = nodes.len();
        nodes.push(new_node);
        node_id
    }

    /// Create a new Arc and add it to the network's collection of arcs
    fn add_arc(&self, start_node_id: usize, end_node_id: usize, cost: f32, min_flow: usize,
               max_flow: usize) {
        let new_arc = arc::Arc::new(start_node_id, end_node_id, cost, min_flow, max_flow);
        let mut arcs = self.arcs.borrow_mut();
        self.nodes.borrow()[start_node_id].add_connection(arcs.len());
        arcs.push( new_arc);
    }

    /// Find the shortest path from the network's source node to its sink node, using an adaptation
    /// of the Bellman-Ford algorithm.
    fn find_shortest_path(&self) -> Result<Vec<usize>, FeasibilityError> {
        #[cfg(feature = "profiling")]
        {
            puffin::profile_function!();
        }

        let nodes = self.nodes.borrow();
        let arcs = self.arcs.borrow();
        let num_nodes = nodes.len();

        // Initialize vectors that represent the paths found so far - at start, we have found no
        // paths, so no node has a found predecessor and all nodes are considered infinite distance
        // from the source, except for the source itself. Node IDs are sequential usize that start
        // from zero to enable using them as indices in these vectors.
        let mut distances = vec![f32::INFINITY; num_nodes];
        distances[0] = 0.0;
        let mut predecessors: Vec<Option<usize>> = vec![None; num_nodes];

        // Search for shortest path, starting from the source.
        let mut nodes_updated = vec![0]; // stores ID numbers
        let mut num_iterations = 0_usize;
        while nodes_updated.len() > 0 && num_iterations < num_nodes {
            let nodes_to_search_from = nodes_updated.clone();
            nodes_updated.clear();

            // for each node updated in the last iteration, see if any of its existing connections
            // result in a shorter path to any other node than what's been found so far
            for node_id in &nodes_to_search_from {
                let node = &nodes[*node_id];
                for connected_arc_id in node.get_connections().iter() {
                    let connected_arc = &arcs[*connected_arc_id];
                    let connected_node_id = connected_arc.get_end_node_id();
                    // calculate distances
                    let cur_dist = distances[connected_node_id];
                    let dist_to_here = distances[*node_id];
                    let dist_from_here = connected_arc.get_cost();

                    if dist_to_here + dist_from_here < cur_dist {
                        // found a shorter path to the connected node
                        distances[connected_node_id] = dist_to_here + dist_from_here;
                        predecessors[connected_node_id] = Some(*node_id);
                        if connected_node_id != 1 {
                            // omit arcs leaving the sink, as these arcs cannot be part of a path to
                            // the sink (else it would be a walk instead of a path) and their
                            // representation within the code is an imperfect mirror of the residual
                            // network for the sake of keeping their data in memory
                            nodes_updated.push(connected_node_id);
                        }
                    }
                }
            }

            num_iterations += 1;
            // eliminate duplicated entries to make sure we only search once before an update
            nodes_updated.sort();
            nodes_updated.dedup();
        }

        // if number of iterations exceeds number of nodes, there's a bug
        if num_iterations >= num_nodes {
            panic!("Negative cycle detected - this can't happen in the algorithm this code \
                   attempts to implement, so there must be a bug.");
        }
        if predecessors[1].is_none() {
            return Err(FeasibilityError { message: "Unable to assign all workers!".to_string() });
        }

        // construct path backwards; unwrap won't panic because the vector is never empty
        let mut path = vec![1];
        while let Some(node_id) = predecessors[*path.last().unwrap()] {
            path.push(node_id);
        }

        // confirm the last node found was the source - if not, there's a bug
        if !(*path.last().unwrap() == 0) {
            panic!("Path does not start at source!")
        }

        path.reverse();
        Ok(path)
    }

    /// Push flow down each arc in a path.
    fn push_flow_down_path(&self, path: &Vec<usize>) {
        #[cfg(feature = "profiling")]
        {
            puffin::profile_function!();
        }

        for node_pair in path.windows(2) {
            let arc = self.find_connecting_arc_id(node_pair[0], node_pair[1])
                .expect("Can't find an arc that's part of the path!");
            let arc_inverted = self.arcs.borrow()[arc].push_flow(self.min_flow_satisfied.get());
            if arc_inverted {
                let nodes = self.nodes.borrow();
                nodes[node_pair[0]].remove_connection(arc);
                nodes[node_pair[1]].add_connection(arc);
            }
        }
    }

    /// The second phase of minimum cost augmentation starts with all tasks having their minimum
    /// requirement satisfied, and allows further assignment of all remaining workers up to the max
    /// for each task. This method resets all arcs touching the sink to account for the
    /// corresponding changes in the residual network.
    fn reset_arcs_for_second_phase(&self) {
        #[cfg(feature = "profiling")]
        {
            puffin::profile_function!();
        }

        let nodes = self.nodes.borrow();
        let connections = nodes[1].get_connections().clone();
        self.min_flow_satisfied.set(true);
        for connection in connections {
            let arc = &self.arcs.borrow()[connection];
            let arc_inverted = arc.update_for_second_phase();
            if arc_inverted {
                nodes[arc.get_end_node_id()].remove_connection(connection);
                nodes[arc.get_start_node_id()].add_connection(connection);
            }
        }
    }

    /// Find the ID of the arc that connects the two identified nodes, if any
    fn find_connecting_arc_id(&self, start_node_id: usize, end_node_id: usize) -> Option<usize> {
        #[cfg(feature = "profiling")]
        {
            puffin::profile_function!();
        }

        self.nodes.borrow()[start_node_id].get_connections().iter()
            .map(|c| *c)
            .find(|c| self.arcs.borrow()[*c].get_end_node_id() == end_node_id)
    }
}

#[cfg(test)]
impl Network {
    /// Get total distance of a path by adding the costs of each arc in the path.
    fn get_path_cost(&self, path: &Vec<usize>) -> f32 {
        path.windows(2)
            .map(|node_pair| {
                let arcs = self.arcs.borrow();
                for arc_id in self.nodes.borrow()[node_pair[0]].get_connections().iter() {
                    if arcs[*arc_id].get_end_node_id() == node_pair[1] {
                        return arcs[*arc_id].get_cost();
                    }
                }
                panic!("No arc found from {} to {}", node_pair[0], node_pair[1])
            })
            .sum()
    }
}
