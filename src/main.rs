mod plot;
use plot::histogram;

mod read_data;
use read_data::{read_graph,read_feat};

mod graph;
use graph::{Graph, compute_and_print_distance_bfs, mark_component_bfs, num_node_list};

mod stats;
use stats::{statistics};

use std::{process,env};
use std::collections::HashMap;


fn main() {
   //user must enter a feature number of interest
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
    eprintln!("Need Feature Number");
    process::exit(1);
    }
   
    //check feature number type
   let feature_number = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a valid feature number");
            process::exit(1);
        }
    };

   //calculate network statistics
   let (list_of_edges,node_set) = read_graph(&"facebook/facebook_combined.txt").unwrap();
   let edge_count = &list_of_edges.len();
   let max_node = *&node_set.iter().max().unwrap() + 1;
   let network_size = &node_set.len();

   println!("\nThe maximum node number is {:?}", max_node);
   println!("\nThe network size is {:?}", network_size);
   println!("\nThe total number of edges is {:?}\n\n\n\n", edge_count);
    

    //calculate network degree statistics
   let graph = Graph::create_undirected(max_node, &list_of_edges);
   let mut degree_list = num_node_list(&graph.outedges);
   let (mean, minimum, quantile50, maximum, std) = statistics(&mut degree_list).unwrap();

   println!("\nThe mean number of friends each user has is {:?}", mean);
   println!("\nThe median number of friends each user has is {:?}", quantile50);
   println!("\nThe minimum number of friends each user has is {:?}", minimum);
   println!("\nThe maximum number of friends each user has is {:?}", maximum);
   println!("\nThe standard deviation of number of friends each user has is {:?}\n\n\n\n", std);
   
   //visualize degree distribution
   let _ = histogram(&degree_list, 
                     &"image/output1.png",
                     250,
                     120,
                     "Number of Connections",
                     "",
                     "Histogram for Number of Connections",
                     240);


    //check network connective components
   let mut component: Vec<Option<usize>> = vec![None;graph.n];
   let mut component_count = 0;
   
   for v in 0..graph.n {
       if let None = component[v] {
           component_count += 1;
           mark_component_bfs(v, &graph, &mut component, component_count);
       }
   };
   println!("\nThe network has a total of {:?} component(s)", component_count);

   //check pair distance for all nodes in the network
   let mut distance_matrix: Vec<Vec<usize>> = Vec::new();
   for i in 0..graph.n{
    distance_matrix.push(compute_and_print_distance_bfs(i, &graph).into_iter()
                                                                  .map(|option| option.unwrap()) 
                                                                  .collect());
   }

   //calculte paired distance statistics
   let mut flatten_distance_matrix: Vec<usize> = distance_matrix.clone().into_iter().flatten().collect();
   let (mean, minimum, quantile50, maximum, std) = statistics(&mut flatten_distance_matrix).unwrap();
   println!("\nThe mean of inter-connection distance is {:?}", mean);
   println!("\nThe median of inter-connection distance is {:?}", quantile50);
   println!("\nThe minimum of inter-connection distance is {:?}", minimum);
   println!("\nThe maximum of inter-connection distance is {:?}", maximum);
   println!("\nThe standard deviation of inter-connection distance is {:?}\n\n\n\n", std);
   
   //visualize paired distances distribution
   let _ = histogram(& flatten_distance_matrix, 
                     &"image/output2.png",
                     9,
                     6500000,
                     "Distance",
                     "",
                     "Histogram for Distances Between Any Two Nodes in the Network",
                     20);


    //read feature data from different ego network
    let mut feature_map: HashMap<usize, usize> = HashMap::new();
    let ego_num = vec![0, 107, 348, 414, 686, 698, 1684, 1912, 3437, 3980];

    for ego in &ego_num {
        let result = read_feat(*ego,feature_number);
        match result {
            Ok(feat) => {feature_map.extend(feat);},
            Err(e) => {println!("\n{:?}",e)},
        };
    }
    println!("\n\n\n");


    //calculate paired distance for network in separate features
    let mut distance_matrix_0: Vec<Vec<usize>> = Vec::new();
    let mut distance_matrix_1: Vec<Vec<usize>> = Vec::new();

    for (key, value) in feature_map.iter() {
        if *value == 0 {distance_matrix_0.push(distance_matrix[*key].clone())}
        if *value == 1 {distance_matrix_1.push(distance_matrix[*key].clone())}
    }

    let mut flatten_distance_matrix_0: Vec<usize>  = distance_matrix_0.clone().into_iter().flatten().collect();
    let mut flatten_distance_matrix_1: Vec<usize>  = distance_matrix_1.clone().into_iter().flatten().collect();
    
    //calculate paired distance statistics for feature network 1
    let (mean0, minimum0, quantile500, maximum0, std0) = statistics(&mut flatten_distance_matrix_0).unwrap();
    println!("\nThe mean of inter-connection distance for feature0 is {:?}", mean0);
    println!("\nThe median of inter-connection distance for feature0 is {:?}", quantile500);
    println!("\nThe minimum of inter-connection distance for feature0 is {:?}", minimum0);
    println!("\nThe maximum of inter-connection distance for feature0 is {:?}", maximum0);
    println!("\nThe standard deviation of inter-connection distance for feature0 is {:?}\n\n\n\n", std0);
   
    //visualize paired distance dsitribution for feature network 1
    let _ = histogram(&flatten_distance_matrix_0, 
                      &"image/output3.png",
                      9,
                      4500000,
                      "Distance of Interconnections",
                      "",
                      "Histogram for Distances of Interconnections for Feature 0 Users",
                      20);
    
    //visualize paired distance dsitribution for feature network 2
    let (mean1, minimum1, quantile501, maximum1, std1) = statistics(&mut flatten_distance_matrix_1).unwrap();
    println!("\nThe mean of inter-connection distance for feature1 is {:?}", mean1);
    println!("\nThe median of inter-connection distance for feature1 is {:?}", quantile501);
    println!("\nThe minimum of inter-connection distance for feature1 is {:?}", minimum1);
    println!("\nThe maximum of inter-connections distance for feature1 is {:?}", maximum1);
    println!("\nThe standard deviation of inter-connection distance for feature1 is {:?}\n\n\n\n", std1);
    
    //visualize paired distance dsitribution for feature network 2
    let _ = histogram(&flatten_distance_matrix_1,
                     &"image/output4.png",
                     9,
                     4500000,
                     "Distance of Interconnections",
                     "",
                     "Histogram for Distance of Interconnections for Feature 1 Users",
                     20);
}
   



   
    

    



