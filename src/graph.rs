
use std::collections::HashMap;

// builds graph from edges in csv
// function takes reference to a vector of tuples (edges in graph) and returns a hasmap where keys are vertices and values are vectors of adjacent vertices
pub fn build_graph(edges: &[(u32, u32)]) -> HashMap<u32, Vec<u32>>{
    //create empty hashmap "graph"
    let mut graph = HashMap::new();
    
    for &(source, target) in edges{
        graph.entry(source).or_insert(Vec::new()).push(target);
        graph.entry(target).or_insert(Vec::new()).push(source);

    }
    graph

}
