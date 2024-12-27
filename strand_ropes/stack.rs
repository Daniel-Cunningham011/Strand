use crate::Node;

#[derive(Clone)]
pub struct NodeStack { 
    vec: Vec<Node>,
    size: usize,
}

impl NodeStack { 
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            size: 0
        }
    }
    
    pub fn push(&mut self, n: Node) {
        self.vec.push(n);
        self.size = self.size + 1;
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.size = self.size - 1;
        self.vec.pop() 
    }

    pub fn empty(&self) -> bool { 
        self.size <= 0
    }
}
