use std::{
    collections::{HashMap, HashSet},
    fs,
};

// Not sure how to safe DFS calculations without a global variable :(
static mut SHORTEST_ROUTE_COST: u32 = 0;
static mut LONGEST_ROUTE_COST: u32 = 0;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut graph = Graph::new();
    for line in contents.lines() {
        let (from, to, dist) = parser(line);
        graph.add_bidirectional_edge(from, to, dist);
    }
    let all_nodes = graph.get_nodes();
    let graph_size = all_nodes.len();
    let mut min_cost = 999999;
    let mut max_cost = 0;
    for node in all_nodes {
        unsafe {
            SHORTEST_ROUTE_COST = 999999;
            LONGEST_ROUTE_COST = 0;
            let mut path = HashSet::new();
            dfs(&graph, &mut path, node, 0, graph_size);
            min_cost = std::cmp::min(SHORTEST_ROUTE_COST, min_cost);
            max_cost = std::cmp::max(LONGEST_ROUTE_COST, max_cost);
        }
    }
    println!("Min cost: {:?}", min_cost);
    println!("Max cost: {:?}", max_cost);
}

fn dfs<'a>(
    graph: &'a Graph,
    path: &mut HashSet<&'a str>,
    node: &'a str,
    current_cost: u32,
    graph_size: usize,
) {
    if path.contains(node) {
        return;
    }
    path.insert(node);
    if path.len() == graph_size {
        unsafe {
            SHORTEST_ROUTE_COST = std::cmp::min(SHORTEST_ROUTE_COST, current_cost);
            LONGEST_ROUTE_COST = std::cmp::max(LONGEST_ROUTE_COST, current_cost);
        }
        path.remove(node);
        return;
    }
    let edges = graph.edges_from(node);
    for edge in edges {
        dfs(
            graph,
            path,
            edge.destination,
            current_cost + edge.distance,
            graph_size,
        );
    }
    path.remove(node);
}

#[derive(Debug)]
struct Graph<'a> {
    graph: HashMap<&'a str, Vec<Edge<'a>>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            graph: HashMap::new(),
        }
    }

    fn add_bidirectional_edge(&mut self, node1: &'a str, node2: &'a str, dist: u32) {
        if !self.graph.contains_key(node1) {
            self.graph.insert(node1, Vec::new());
        }
        if !self.graph.contains_key(node2) {
            self.graph.insert(node2, Vec::new());
        }
        let edges: &mut Vec<Edge> = self.graph.get_mut(node1).unwrap();
        edges.push(Edge::new(node2, dist));

        let edges: &mut Vec<Edge> = self.graph.get_mut(node2).unwrap();
        edges.push(Edge::new(node1, dist));
    }

    fn get_nodes(&self) -> Vec<&'a str> {
        let mut nodes = Vec::new();
        for k in self.graph.keys() {
            nodes.push(*k);
        }
        nodes
    }

    fn edges_from(&self, node: &str) -> &Vec<Edge> {
        self.graph.get(node).unwrap()
    }
}

#[derive(Debug)]
struct Edge<'a> {
    distance: u32,
    destination: &'a str,
}

impl<'a> Edge<'a> {
    fn new(destination: &'a str, distance: u32) -> Self {
        Edge {
            distance,
            destination,
        }
    }
}

fn parser(line: &str) -> (&str, &str, u32) {
    let splits1: Vec<&str> = line.split("=").collect();
    let distance: u32 = splits1[1].trim().parse().unwrap();
    let cities: Vec<&str> = splits1[0].split("to").collect();
    (cities[0].trim(), cities[1].trim(), distance)
}
