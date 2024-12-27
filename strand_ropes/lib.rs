#![allow(unused)]
mod stack;
use stack::NodeStack;

trait TreeComponent {
    fn left(&self) -> Option<impl TreeComponent>;
    fn right(&self) -> Option<impl TreeComponent>;
    fn is_leaf(&self) -> bool;
}

#[derive(Clone, PartialEq)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: String,
    size: usize,
}

#[allow(refining_impl_trait)]
impl TreeComponent for Node {
    fn left(&self) -> Option<Node> {
        if let Some(x) = &self.left {
            return Some(*(x.clone()));
        }
        return None
    }

    fn right(&self) -> Option<Node> {
        if let Some(x) = &self.right {
            return Some(*(x.clone()));
        }
        return None
    }

    fn is_leaf(&self) -> bool {
        if let Some(_x) = self.left() {
            return false;
        }
        else if let Some(_y) = self.right() {
            return false;
        }
        true
    }

}

struct Rope {
    head: Option<Box<Node>>,
    size: usize,
}

#[allow(refining_impl_trait)]
impl TreeComponent for Rope {
    fn left(&self) -> Option<Node> {
        if let Some(x) = &self.head {
            if let Some(y) = x.left() {
                return Some(y);
            };
        }
        None
    }

    fn right(&self) -> Option<Node> {
        if let Some(x) = &self.head {
            if let Some(y) = x.right() {
                return Some(y);
            }
        }
        None
    }

    fn is_leaf(&self) -> bool {
        if let None = self.left() {
            if let None = self.right() {
                return true;
            }
        }
        false
    }
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        } 
    }
    pub fn head(&self) -> Option<Node> {
        if let Some(x) = &self.head {
            return Some(*x.clone())
        }
        None
    }

    pub fn collect(&self) -> Vec<Node> {
        let mut stack = NodeStack::new(); 
        let mut head = match self.head() {
            None => return Vec::new(),
            Some(x) => Some(x)
        };
        
        let data: Vec<Node> = Vec::new();
        self.traverse_left(&mut stack.clone(), &mut data.clone(), head); 
        while !stack.empty() {
            match stack.pop() {
                Some(node) =>  {
                   match node.right() {
                        Some(right_node) => {
                            self.traverse_left(&mut stack.clone(), &mut data.clone(), Some(right_node));
                        },
                        None => {}
                   }
                },
                None => {}
            }
        }
        return data; 
    }

    pub fn rebalance(&mut self) -> Self {
       Self::new() 
    }

    pub fn depth(&self) -> usize {
        if let Some(head_node) = self.head() {
           let left_depth = match head_node.left() {
                Some(left_node) => 1 + self.depth_helper(left_node),
                None => 0
           };
           let right_depth = match head_node.right() {
                Some(right_node) => 1 + self.depth_helper(right_node),
                None => 0
           };
           let maximum = match left_depth > right_depth {
                true => left_depth,
                false => right_depth
           };
           return maximum;
        }
        0
    }

    fn depth_helper(&self, current_node: Node) -> usize {
        let left_depth = match current_node.left() {
            Some(left_node) => 1 + self.depth_helper(left_node),
            None => 0
        };
        let right_depth = match current_node.right() {
            Some(right_node) => 1 + self.depth_helper(right_node),
            None => 0
        };
        let maximum = match left_depth > right_depth {
            true => left_depth,
            false => right_depth
        };
        return maximum;
    }

    // 2^n = height =>  log_2(height) = n

    fn isBalanced(&self) -> bool {
        let depth = self.depth();
        true     
    }

    fn traverse_left(&self, stack: &mut NodeStack, data: &mut Vec<Node>, current_node: Option<Node>) {
        let mut finished_traverse = false; 
        let mut head = current_node;
        while finished_traverse != true {
            stack.push(head.clone().unwrap());
            head = match self.left() {
                Some(node) => Some(node),
                None => {finished_traverse = true; head}, 
            };
        }
        data.push(head.unwrap()); 
    }
}
