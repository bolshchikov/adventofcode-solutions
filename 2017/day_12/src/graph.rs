use std::collections::HashMap;
use input;

fn build_graph(read_input: &Fn() -> Vec<String>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let lines = read_input();
    for l in lines.iter() {
        let nodes = input::parse_line(l);
        graph.insert(nodes[0], nodes[1..].to_vec());
    }

    graph
}

pub fn build_example_graph() -> HashMap<i32, Vec<i32>> {
    build_graph(&input::get_example_input)
}

pub fn build_input_graph() -> HashMap<i32, Vec<i32>> {
    build_graph(&input::get_input)
}

pub fn traverse(graph: &HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited: Vec<i32> = Vec::new();
    let mut path: Vec<i32> = vec![start];

    while path.len() != 0 {
        let node = path.remove(0);
        if !visited.contains(&node) {
            let children = graph.get(&node).unwrap();
            visited.push(node);
            path.extend_from_slice(&children);
        }
    }
    visited
}

pub fn trees(graph: &HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visited: Vec<i32> = Vec::new();
    let mut trees: Vec<Vec<i32>> = Vec::new();

    let mut keys: Vec<&i32> = graph.keys().collect();
    keys.sort();

    for id in keys {
        if !visited.contains(&id) {
            let tree = traverse(graph, *id);
            visited.extend_from_slice(&tree);
            trees.push(tree);
        }
    }
    trees
}
