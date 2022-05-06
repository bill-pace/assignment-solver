/// A generic node in the network, used to represent source/sink, workers, and tasks.
/// Each node has an ID number sequentially generated on construction, and a collection of other
/// ID numbers corresponding to the nodes that it connects to via existing arcs. Note that since
/// this network is directed, the connected nodes do not point back to this node.
pub struct Node {
    id: usize,
    connected_nodes: Vec<usize>,
}

impl Node {
    /// Create new Node with given ID
    pub fn new(id: usize) -> Node {
        Node { id, connected_nodes: Vec::new() }
    }

    /// Get number of connected nodes
    pub fn get_num_connected_nodes(&self) -> usize {
        self.connected_nodes.len()
    }

    /// Get ID of first connected node, if any
    pub fn get_first_connected_node_id(&self) -> Option<usize> {
        if self.connected_nodes.len() > 0 {
            Some(self.connected_nodes[0])
        } else {
            None
        }
    }
}
