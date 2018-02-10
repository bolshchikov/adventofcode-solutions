use std::thread;

mod graph;
mod input;

fn main() {
    let traverse = thread::spawn(|| {
        let pipes = graph::build_input_graph();
        let visited = graph::traverse(&pipes, 0);
        println!("{:?}", visited.len());
    });

    let groups = thread::spawn(|| {
        let pipes = graph::build_input_graph();
        let trees = graph::trees(&pipes);
        println!("{:?}", trees.len());
    });

    traverse.join();
    groups.join();
}
