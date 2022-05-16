use std::cell::{Ref, RefCell};

/// A generic node in the network, used to represent source/sink, workers, and tasks.
/// Each node has an ID number sequentially generated on construction, and a collection of other
/// ID numbers corresponding to the nodes that it connects to via existing arcs. Note that since
/// this network is directed, the connected nodes do not point back to this node.
pub struct Node {
    connected_arcs: RefCell<Vec<usize>>
}

impl Node {
    /// Create new Node with given ID
    pub fn new() -> Node {
        Node { connected_arcs: RefCell::new(Vec::new()) }
    }

    /// Get number of connected arcs
    pub fn get_num_connections(&self) -> usize {
        #[cfg(test)]
        {
            puffin::profile_function!();
        }

        self.connected_arcs.borrow().len()
    }

    /// Get ID of first connected arc, if any
    pub fn get_first_connected_arc_id(&self) -> Option<usize> {
        match self.connected_arcs.borrow().first() {
            Some(v) => Some(*v),
            None => None
        }
    }

    /// Create new connection
    pub fn add_connection(&self, arc_id: usize) {
        #[cfg(test)]
        {
            puffin::profile_function!();
        }

        if !self.connected_arcs.borrow().contains(&arc_id) {
            self.connected_arcs.borrow_mut().push(arc_id);
        }
    }

    /// Remove existing connection. Assume that the connection can be listed only once.
    pub fn remove_connection(&self, arc_id: usize) {
        #[cfg(test)]
        {
            puffin::profile_function!();
        }

        let idx = self.connected_arcs.borrow().iter()
            .position(|x| *x == arc_id).unwrap();
        self.connected_arcs.borrow_mut().swap_remove(idx);
    }

    /// Returns a clone of the list of connected arc IDs.
    pub fn get_connections(&self) -> Ref<Vec<usize>> {
        #[cfg(test)]
        {
            puffin::profile_function!();
        }

        self.connected_arcs.borrow()
    }
}
