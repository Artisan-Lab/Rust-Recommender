struct Node {
    links: Vec<usize>,
    dirty: usize,
}

struct Graph {
    nodes: Vec<Node>,
}

fn main() {
    let mut graph = Graph {
        nodes: Vec::<Node>::new(),
    };

    // Add some nodes
    
    for link in graph.nodes[0].links.iter() {
        graph.nodes[*link].dirty = 1
    }
}