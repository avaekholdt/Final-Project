
use std::collections::{HashMap, VecDeque};

//calcualte shortest path between two vertices in graph

//function takes in reference to a graph (hashmap) which represents neighbors of each vertex
pub fn shortest_path(graph: &HashMap<u32, Vec<u32>>, source: u32, target: u32)-> Option<Vec<u32>>{
    let mut queue = VecDeque::new(); //queue to use for BFS
    let mut visited = HashMap::new(); //keep track of vertices already visited during BFS
    let mut prev = HashMap::new(); //stores parent vertex of each vertex to reconstruct shortest path 

    queue.push_back(source);
    visited.insert(source, 0);

    while let Some(vertex) = queue.pop_front(){
        if vertex == target{
            break;
        }
        if let Some(neighbors) = graph.get(&vertex){
            for &neighbor in neighbors{
                if !visited.contains_key(&neighbor){
                    visited.insert(neighbor, visited[&vertex] + 1);
                    prev.insert(neighbor, vertex);
                    queue.push_back(neighbor);
                }
            }
        }
    }
    if visited.contains_key(&target){
        let mut path = Vec::new();
        let mut vertex = target;
        while vertex != source{
            path.push(vertex);
            vertex = prev[&vertex];
        }
        path.push(source);
        path.reverse();
        Some(path)
    } else{
        None
    }
}