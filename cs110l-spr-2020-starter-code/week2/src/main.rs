struct Node {
    value: u32,
    next: Option<Box<Node>>, // no null pointers in Rust!
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl Node {
    fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node {value: value, next: next}
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {head: None, size: 0}
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    // pub fn get_size(&self) -> usize {
    //     (*self).size
    // }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    // pub fn is_empty(&self) -> bool {
    //     self.get_size() == 0
    // }
    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}

fn main(){
    let mut list:LinkedList = LinkedList::new();
    list.push(5);
    println!("{}",list.is_empty());
}