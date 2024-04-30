mod functions;

use petgraph::graph::UnGraph;
use petgraph::dot::{Dot, Config};
use std::path::Path;

fn main() {
    let path = Path::new("/Users/itaimerlin/Downloads/facebook_large/musae_facebook_features.json");
    let people = functions::load_data(&path).expect("Could not load data");

    let mut graph = UnGraph::<String, &str>::new_undirected();

    // Build nodes and edges based on shared interests
    let mut name_to_node = std::collections::HashMap::new();
    for person in &people {
        let (name, interests) = functions::parse_person(person).expect("Error parsing person data");
        let node_id = graph.add_node(name.clone());
        name_to_node.insert(name, (node_id, interests));
    }

    // Create edges for shared interests
    for (name_a, (node_id_a, interests_a)) in &name_to_node {
        for (name_b, (node_id_b, interests_b)) in &name_to_node {
            if name_a != name_b && !interests_a.is_disjoint(interests_b) {
                graph.add_edge(*node_id_a, *node_id_b, "shared interest");
            }
        }
    }

    // Output the graph in DOT format
    println!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());
}
