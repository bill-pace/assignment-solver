/// A generic node in the network, used to represent source/sink, workers, and tasks.
/// Each node has an ID number sequentially generated on construction, and a collection of other
/// ID numbers corresponding to the nodes that it connects to via existing arcs. Note that since
/// this network is directed, the connected nodes do not point back to this node.
pub struct Node {
    connected_nodes: Vec<usize>,
}

impl Node {
    /// Create new Node with given ID
    pub fn new() -> Node {
        Node { connected_nodes: Vec::new() }
    }

    /// Get number of connected nodes
    pub fn get_num_connected_nodes(&self) -> usize {
        self.connected_nodes.len()
    }

    /// Get ID of first connected node, if any
    // TODO: use self.connected_nodes.first() instead
    pub fn get_first_connected_node_id(&self) -> Option<usize> {
        if self.connected_nodes.len() > 0 {
            Some(self.connected_nodes[0])
        } else {
            None
        }
    }

    /// Create new connection
    pub fn add_connection(&mut self, node_id: usize) {
        if !self.connected_nodes.contains(&node_id) {
            self.connected_nodes.push(node_id);
        }
    }

    /// Remove existing connection. Assume that the connection can be listed only once.
    pub fn remove_connection(&mut self, node_id: usize) {
        let idx = self.connected_nodes.iter().position(|x| *x == node_id).unwrap();
        self.connected_nodes.swap_remove(idx);
    }

    /// Returns a reference to the list of connected node IDs.
    pub fn get_connections(&self) -> &Vec<usize> {
        &self.connected_nodes
    }
}
