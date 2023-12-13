use std::collections::{VecDeque};

//reference: please note that part of the code in this module are referenced from DS210, Professor Leonidas Kontothanassis's lecture 27/28 slides
//define a Graph structure with a usize indicating the number of nodes (n) and a adjacency list to represent directed outedges
#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: Vec<Vec<usize>>,
}

//reverse the direction of edges in a given list of edges
pub fn reverse_edges(list: &Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

//count number of connections for each node in the graph and return as a vector
pub fn num_node_list(adj_list: &Vec<Vec<usize>>) -> Vec<usize> {
    adj_list.iter().map(|inner_vec| inner_vec.len()).collect()
}

impl Graph {
    //add directed edges to the graph
    pub fn add_directed_edges(&mut self, edges:&Vec<(usize,usize)>) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    //sort the outedges lists for each node in the graph
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    //create a directed graph given the number of nodes and a list of edges
    pub fn create_directed(n:usize,edges:&Vec<(usize,usize)>) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }

    //create a undirected graph given the number of nodes and a list of edges
    pub fn create_undirected(n:usize,edges:&Vec<(usize,usize)>) -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
}

//compute and print the distance from a start node to all other nodes using Breadth-First Search
pub fn compute_and_print_distance_bfs(start: usize, graph: &Graph) -> Vec<Option<usize>> {
    let mut distance: Vec<Option<usize>> = vec![None;graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    distance
}

//mark connected components in a graph using Breadth-First Search
pub fn mark_component_bfs(vertex:usize, graph:&Graph, component:&mut Vec<Option<usize>>, component_no:usize) {
    component[vertex] = Some(component_no);
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(vertex);
    
    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                queue.push_back(*w);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_outer_vector() {
        let adj_list: Vec<Vec<usize>> = Vec::new();
        assert_eq!(num_node_list(&adj_list), vec![]);
    }

    #[test]
    fn test_outer_vector_with_empty_inner_vectors() {
        let adj_list = vec![vec![], vec![], vec![]];
        assert_eq!(num_node_list(&adj_list), vec![0, 0, 0]);
    }

    #[test]
    fn test_outer_vector_with_various_inner_vector_lengths() {
        let adj_list = vec![vec![1, 2], vec![1, 2, 3], vec![1]];
        assert_eq!(num_node_list(&adj_list), vec![2, 3, 1]);
    }

    #[test]
    fn test_graph_0() {
        let vec: Vec<(usize,usize)> = vec![(0,1),(1,2),(0,2)];
        let graph = Graph::create_directed(3, &vec);
        assert_eq!(graph.outedges[0], vec![1,2]);
    }

    #[test]
    fn test_graph_1() {
        let vec: Vec<(usize,usize)> = vec![(0,1),(1,2),(0,2)];
        let graph = Graph::create_directed(3, &vec);
        assert_eq!(graph.outedges[1], vec![2]);
    }

    #[test]
    fn test_graph_2() {
        let vec: Vec<(usize,usize)> = vec![(0,1),(1,2),(0,2)];
        let graph = Graph::create_undirected(3, &vec);
        assert_eq!(graph.outedges[2], vec![0,1]);
    }
}
