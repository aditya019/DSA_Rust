 use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
};

pub struct Graph {
    adjecency_list: HashMap<i32, RefCell<HashSet<i32>>>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            adjecency_list: HashMap::new(),
        }
    }
    fn add_vertex(&mut self, v: i32) {
        self.adjecency_list.insert(v, RefCell::new(HashSet::new()));
    }
    fn add_edge(&mut self, s: i32, d: i32) {
        if !self.adjecency_list.contains_key(&s) || !self.adjecency_list.contains_key(&d) {
            panic!("Node(s) not present in the graph");
        }
        self.adjecency_list.get(&s).unwrap().borrow_mut().insert(d);
        self.adjecency_list.get(&d).unwrap().borrow_mut().insert(s);
    }
    fn print(&self) {
        for (i, set) in &self.adjecency_list {
            print!("{i} : ");
            for j in set.borrow().iter() {
                print!("{j} ,");
            }
            println!();
        }
    }
    fn dfs(&self, s: i32) {
        if !self.adjecency_list.contains_key(&s) {
            panic!("Node not present");
        }
        let mut stack = Vec::new();
        stack.push(s);
        let mut visited = HashSet::new();
        println!("DFS of the graph");
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            print!("{node} ");
            visited.insert(node);
            for i in self.adjecency_list.get(&node).unwrap().borrow().iter() {
                if !stack.contains(i) && !visited.contains(i) {
                    stack.push(*i);
                }
            }
        }
        println!();
    }
    fn dfs_recursive(&self, s: i32) {
        if !self.adjecency_list.contains_key(&s) {
            panic!("Node not present");
        }
        let mut visited = HashSet::new();
        self.dfs_recursive_util(s, &mut visited);
        println!();
    }
    fn dfs_recursive_util(&self, s: i32, visited: &mut HashSet<i32>) {
        if visited.contains(&s) {
            return;
        }
        print!("{s} ");
        visited.insert(s);
        for i in self.adjecency_list.get(&s).unwrap().borrow().iter() {
            self.dfs_recursive_util(*i, visited);
        }
    }
    fn bfs(&self, s: i32) {
        if !self.adjecency_list.contains_key(&s) {
            panic!("Node not resent");
        }
        println!("BFS of graph");
        let mut q = VecDeque::new();
        q.push_back(s);
        let mut visited = HashSet::new();
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            visited.insert(node);
            print!("{node} ");
            for i in self.adjecency_list.get(&node).unwrap().borrow().iter() {
                if !q.contains(i) && !visited.contains(i) {
                    q.push_back(*i);
                }
            }
        }
        println!();
    }
}

fn main() {
    let mut graph = Graph::new();
    for i in 1..=5 {
        graph.add_vertex(i);
    }
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(4, 3);
    graph.add_edge(4, 2);
    graph.add_edge(4, 5);
    graph.print();
    graph.dfs(3);
    println!("dfs using recursion");
    graph.dfs_recursive(3);
    graph.bfs(3);
}
