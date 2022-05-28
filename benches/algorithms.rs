#[macro_use]
extern crate bencher;

use std::cell::Cell;
use bencher::Bencher;

fn node_path(bench: &mut Bencher) {
    let mut predecessors: Vec<Option<usize>> = vec![None; 100];
    predecessors[1] = Some(79);
    predecessors[79] = Some(56);
    predecessors[56] = Some(63);
    predecessors[63] = Some(71);
    predecessors[71] = Some(25);
    predecessors[25] = Some(99);
    predecessors[99] = Some(16);
    predecessors[16] = Some(85);
    predecessors[85] = Some(84);
    predecessors[84] = Some(0);

    bench.iter(|| {
        let mut path = vec![1];
        while let Some(node_id) = predecessors[*path.last().unwrap()] {
            path.push(node_id);
        }
    })
}

struct Arc {
    start_node: Cell<usize>
}

impl Arc {
    fn get_start_node_id(&self) -> usize {
        self.start_node.get()
    }
}

fn arc_path(bench: &mut Bencher) {
    let mut predecessors: Vec<Option<usize>> = vec![None; 100];
    let mut arcs: Vec<Arc> = Vec::new();
    arcs.push(Arc { start_node: Cell::new(79) });
    predecessors[1] = Some(0);
    arcs.push(Arc { start_node: Cell::new(56) });
    predecessors[79] = Some(1);
    arcs.push(Arc { start_node: Cell::new(63) });
    predecessors[56] = Some(2);
    arcs.push(Arc { start_node: Cell::new(71) });
    predecessors[63] = Some(3);
    arcs.push(Arc { start_node: Cell::new(25) });
    predecessors[71] = Some(4);
    arcs.push(Arc { start_node: Cell::new(99) });
    predecessors[25] = Some(5);
    arcs.push(Arc { start_node: Cell::new(16) });
    predecessors[99] = Some(6);
    arcs.push(Arc { start_node: Cell::new(85) });
    predecessors[16] = Some(7);
    arcs.push(Arc { start_node: Cell::new(84) });
    predecessors[85] = Some(8);
    arcs.push(Arc { start_node: Cell::new(0) });
    predecessors[84] = Some(9);

    bench.iter(|| {
        let mut path = vec![predecessors[1].unwrap()];
        while let Some(arc_id) = predecessors[arcs[*path.last().unwrap()].get_start_node_id()] {
            path.push(arc_id);
        }
    })
}

benchmark_group!(path_construction, node_path, arc_path);
benchmark_main!(path_construction);
