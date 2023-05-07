
use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::collections::HashMap;
use std::path::Path;

mod graph;
mod distance;

#[derive(Debug)]
enum CsvError{
    IoError(std::io::Error),
    //ParseError(String),

}
//open and read file
fn read_csv_file(file_path: &str) -> Result<Vec<(u32, u32)>, CsvError>{
    let path = Path::new(file_path);
    let file = File::open(path).map_err(CsvError::IoError)?;

    let reader = BufReader::new(file);
//skips first 4 lines because it contians headers
    let edges = reader.lines().skip(4).map(|line| {
        let line = line.unwrap();
        let mut iter = line.split(",");
        let source = iter.next().unwrap().parse::<u32>().unwrap();
        let target = iter.next().unwrap().parse::<u32>().unwrap();
        Ok((source, target))
    }).collect::<Result<Vec<_>, _>>()?;
    Ok(edges)
}

fn main(){
    let edges = read_csv_file("edges.csv").expect("Error reading file");
    let graph = graph::build_graph(&edges);

//use distance and num_pairs to calculate the average distance between pair of vertices later 
    let mut total_distance = 0;
    let mut num_pairs = 0;

// iterate over all pairs of vertices in the graph, except those whetre source = target
    for source in graph.keys(){
        for target in graph.keys(){
            if source != target{
                //if shortest path found between source and target, length - 1 is added to total distance
                if let Some(path) = distance::shortest_path(&graph, *source, *target){
                    total_distance += path.len() -1;
                    num_pairs += 1;
                }
            }
        }
    }


    //find average distance between pairs of vertices
    let average_distance = total_distance as f64 / num_pairs as f64;
    println!("Average distance between pairs of vertices in the facebook graph: {}", average_distance);

    //using another graph from twitch.csv
    let edges2 = read_csv_file("twitch.csv").expect("Error reading file");
    let graph2 = graph::build_graph(&edges2);
    
    //to calculate the average distance
    let mut total_distance2 = 0;
    let mut num_pairs2 = 0;

    for source in graph2.keys(){
        for target in graph2.keys(){
            if source != target{
                //if shortest path found bw source and target, length - 1 added to total
                if let Some(path) = distance::shortest_path(&graph2, *source, *target){
                    total_distance2 += path.len()-1;
                    num_pairs2 += 1;
                }
            }
        }
    }

    let average_distance2 = total_distance2 as f64 / num_pairs2 as f64;
    println!("Average distance between pairs of vertices in the second graph: {}", average_distance2);

    let ratio = average_distance / average_distance2;
    println!("The ratio between the average distance in the vertices of facebook graph vs twitch graph: {}", ratio);



}