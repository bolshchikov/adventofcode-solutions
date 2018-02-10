mod graph;
mod input;

fn main() {
    let pipes = graph::build_input_graph();
    let visited = graph::traverse(&pipes, 0);
    println!("{:?}", visited.len());
}
