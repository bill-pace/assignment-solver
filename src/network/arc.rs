use std::cell::Cell;
use std::fmt;

/// An arc that connects two nodes in the network.
/// Each arc tracks the ID numbers of the nodes where it starts and ends, the cost associated with
/// pushing a single unit of flow down the arc, the lower and upper bounds on flow that must/can be
/// pushed down the arc, and the current amount of flow down the arc. Note that the lower flow bound
/// actually represents the flow that must be present in that arc at the point when execution can
/// move from the "satisfy minimum assignment" phase to the "assign all remaining workers" phase.
#[derive(Debug)]
pub struct Arc {
    start_node: Cell<usize>,
    end_node: Cell<usize>,
    cost: Cell<f32>,
    min_flow: usize,
    max_flow: usize,
    current_flow: Cell<usize>,
}

impl Arc {
    /// Create a new Arc
    pub fn new(start_node_id: usize, end_node_id: usize, cost: f32, min_flow: usize,
               max_flow: usize) -> Arc {
        Arc { start_node: Cell::new(start_node_id), end_node: Cell::new(end_node_id),
              cost: Cell::new(cost), min_flow, max_flow, current_flow: Cell::new(0) }
    }

    /// Increment flow along this arc by 1. If flow reaches max, invert the arc to keep the residual
    /// network's representation up-to-date. We don't care to track residuals for any arc that has
    /// max flow greater than 1, because the only arcs that can have max flow greater than 1 in this
    /// network are those that touch the sink. Since we never push flow in a cycle, we will never
    /// decrease the amount of flow in an arc that touches the sink.
    pub fn push_flow(&self, min_flow_satisfied: bool) -> bool {
        self.current_flow.set(self.current_flow.get() + 1);
        let mut inverted = false;
        if min_flow_satisfied {
            if self.current_flow.get() == self.max_flow {
                self.invert();
                inverted = true;
            }
        } else {
            if self.current_flow.get() == self.min_flow {
                self.invert();
                inverted = true;
            }
        }
        inverted
    }

    /// Invert this arc so the residual network's representation stays up-to-date: negate cost, find
    /// new flow bounds, reset the current flow, and flip the start/end node IDs. For the network in
    /// this particular problem, the only arcs whose flow bounds would need to change in the
    /// residual network are those that flow into the sink. Arcs that leave the sink can never be
    /// part of a path to the sink (else the path would include the sink more than once and
    /// therefore be a walk), so we do not actually need to change those values: arcs whose
    /// residuals can actually impact the shortest path algorithm always have 1 max flow.
    fn invert(&self) {
        // flip direction of arc
        self.cost.set(-self.cost.get());
        self.current_flow.set(0); // 0 is accurate for arcs that touch workers, and resetting
                                 // this value here doesn't matter for arcs that don't touch workers

        // switch endpoints
        let temp_id = self.start_node.get();
        self.start_node.set(self.end_node.get());
        self.end_node.set(temp_id);
    }

    /// Get the arc's cost
    pub fn get_cost(&self) -> f32 {
        self.cost.get()
    }

    /// Get the arc's start node id
    pub fn get_start_node_id(&self) -> usize {
        self.start_node.get()
    }

    /// Get the arc's end node id
    pub fn get_end_node_id(&self) -> usize {
        self.end_node.get()
    }

    /// Invert arc for second phase of min cost augmentation, unless it's already at max capacity
    pub fn update_for_second_phase(&self) -> bool {
        if self.min_flow == self.max_flow {
            // nothing to update - this arc is already at max capacity, too
            return false;
        }

        self.invert();
        self.current_flow.set(self.min_flow);
        true
    }
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "start: {} end: {} cost: {} min: {} max: {} flow: {}",
               self.start_node.get(), self.end_node.get(), self.cost.get(),
               self.min_flow, self.max_flow, self.current_flow.get())
    }
}
