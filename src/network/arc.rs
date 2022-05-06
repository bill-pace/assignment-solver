use std::collections::HashMap;
use crate::network::node::Node;

/// An arc that connects two nodes in the network.
/// Each arc tracks the ID numbers of the nodes where it starts and ends, the cost associated with
/// pushing a single unit of flow down the arc, the lower and upper bounds on flow that must/can be
/// pushed down the arc, and the current amount of flow down the arc.
pub struct Arc {
    start_node: usize,
    end_node: usize,
    cost: f32,
    min_flow: usize,
    max_flow: usize,
    current_flow: usize,
}

impl Arc {
    /// Create a new Arc. The mutable reference to the HashMap of nodes is not stored within the
    /// struct, and therefore dropped when the constructor returns.
    pub fn new(start_node_id: usize, end_node_id: usize, cost: f32, min_flow: usize,
               max_flow: usize, nodes: &mut HashMap<usize, Node>) -> Arc {
        nodes.get_mut(&start_node_id).unwrap().add_connection(end_node_id);
        Arc { start_node: start_node_id, end_node: end_node_id, cost,
              min_flow, max_flow, current_flow: 0 }
    }

    /// Increment flow along this arc by 1. If flow reaches max, invert the arc to keep the residual
    /// network's representation up-to-date. We don't care to track residuals for any arc that has
    /// max flow greater than 1, because the only arcs that can have max flow greater than 1 in this
    /// network are those that touch the sink. Since we never push flow in a cycle, we will never
    /// decrease the amount of flow in an arc that touches the sink.
    pub fn push_flow(&mut self, nodes: &mut HashMap<usize, Node>) {
        self.current_flow += 1;
        if self.current_flow == self.max_flow {
            self.invert(nodes);
        }
    }

    /// Invert this arc so the residual network's representation stays up-to-date: negate cost, find
    /// new flow bounds, reset the current flow, and flip the start/end node IDs.
    /// For the network in this particular problem, the only arcs whose flow bounds would need to
    /// change in the residual network are those that flow into the sink. Arcs that leave the sink
    /// can never be part of a path to the sink (else the path would include the sink more than once
    /// and therefore be a walk), so we do not actually need to change those values: arcs whose
    /// residuals can actually impact the shortest path algorithm always have 0 min flow and 1 max.
    fn invert(&mut self, nodes: &mut HashMap<usize, Node>) {
        // flip direction of arc
        self.cost = -self.cost;
        self.current_flow = 0;

        // update endpoints and pass info to the nodes
        nodes.get_mut(&self.start_node).unwrap().remove_connection(self.end_node);
        nodes.get_mut(&self.end_node).unwrap().add_connection(self.start_node);
        let temp_id = self.start_node;
        self.start_node = self.end_node;
        self.end_node = temp_id;
    }

    /// Get the arc's cost
    pub fn get_cost(&self) -> f32 {
        self.cost
    }

    /// Get the arc's start node id
    pub fn get_start_node_id(&self) -> usize {
        self.start_node
    }

    /// Get the arc's end node id
    pub fn get_end_node_id(&self) -> usize {
        self.end_node
    }
}
