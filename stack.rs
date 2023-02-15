//This is a simple implementation of stack in rust

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(v: i32) -> Self {
        Node { val: v, next: None }
    }
}

#[derive(Debug)]
struct List {
    head: Option<Box<Node>>,
}

impl List {
    fn new(val: i32) -> Self {
        let node = Node::new(val);
        List {
            head: Some(Box::new(node)),
        }
    }
    fn push(&mut self, v: i32) {
        let node = Box::new(Node::new(v));
        if self.is_empty() {
            List::new(v);
        }
        let temp = self.head.take();
        self.head = Some(node);
        self.head.as_mut().unwrap().next = temp;
    }
    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let temp = self.head.as_mut().unwrap().next.take();
        let top = self.head.as_ref().unwrap().val;
        self.head = temp;
        Some(top)
    }
    fn len(&self) -> usize {
        if self.head.is_none() {
            return 0;
        }
        let mut res = 0;
        let mut cnode = &self.head;
        while cnode.is_some() {
            cnode = &cnode.as_ref().unwrap().next;
            res += 1;
        }
        res
    }
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    fn print(&self) {
        let mut cnode = &self.head;
        while cnode.is_some() {
            println!("{}", cnode.as_ref().unwrap().val);
            cnode = &cnode.as_ref().unwrap().next;
        }
    }
}

fn main() {
    println!("This my implementation of a Simple Stack in Rust");
    let mut list = List::new(0);
    for i in 1..=5 {
        list.push(i);
    }
    list.print();
    println!("popping the top element");
    let top = list.pop().unwrap();
    println!("the top element : {top}");
    println!("List After popping");
    list.print();
}
