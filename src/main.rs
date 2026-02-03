mod graph;

use graph::DependencyGraph;

fn main() {
    let mut graph = DependencyGraph::new();

    graph.add_module("core", vec![]);
    graph.add_module("math", vec!["core".to_string()]);
    graph.add_module("utils", vec!["core".to_string(), "math".to_string()]);

    graph.print();

    graph.remove_module("math");

    println!("\nAfter removal:");
    graph.print();
}
