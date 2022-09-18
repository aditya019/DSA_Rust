use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Graph {
    adjacency_list: HashMap<i32, Vec<Edge>>,
}

#[derive(Eq)]
struct Edge {
    s: i32,
    d: i32,
    w: usize,
}

impl Edge {
    fn new(s: i32, d: i32, w: usize) -> Self {
        Edge { s, d, w }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.w.partial_cmp(&other.w)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.w.cmp(&other.w)
    }
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    fn add_vertex(&mut self, s: i32) {
        self.adjacency_list.entry(s).or_insert(Vec::new());
    }
    fn add_edge(&mut self, s: i32, d: i32, w: usize) {
        if !self.adjacency_list.contains_key(&s) || !self.adjacency_list.contains_key(&d) {
            panic!("Node(s) not found");
        }
        let s_edge = Edge::new(s, d, w);
        let d_edge = Edge::new(d, s, w);
        self.adjacency_list.get_mut(&s).unwrap().push(s_edge);
        self.adjacency_list.get_mut(&d).unwrap().push(d_edge);
    }
    fn print(&self) {
        for (i, j) in &self.adjacency_list {
            print!("{i} : ");
            for k in j {
                print!("{}--{}--{} , ", k.s, k.w, k.d);
            }
            println!();
        }
    }
    fn prims(&self) -> Vec<&Edge> {
        let mut res = Vec::new();
        let mut heap = BinaryHeap::new();
        let s = self.adjacency_list.keys().next().unwrap();
        let mut visited = HashSet::new();
        visited.insert(*s);
        for i in self.adjacency_list.get(s).unwrap() {
            heap.push(Reverse(i));
        }
        while visited.len() != self.adjacency_list.len() {
            while visited.contains(&heap.peek().unwrap().0.d) {
                heap.pop();
            }
            let e = heap.pop().unwrap().0;
            visited.insert(e.d);
            res.push(e);
            visited.insert(e.d);
            for i in self.adjacency_list.get(&e.d).unwrap() {
                heap.push(Reverse(i));
            }
        }
        res
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_edge(1, 2, 4);
    graph.add_edge(1, 4, 1);
    graph.add_edge(1, 3, 2);
    graph.add_edge(4, 3, 6);
    graph.add_edge(2, 3, 10);
    graph.add_edge(3, 5, 3);
    graph.print();
    let mst = graph.prims();
    for i in mst {
        println!("{}--{}--{}", i.s, i.w, i.d);
    }
}
