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
