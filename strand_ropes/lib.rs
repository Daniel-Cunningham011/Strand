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
    pub fn head(&self) -> Option<Node> {
        if let Some(x) = &self.head {
            return Some(*x.clone())
        }
        None
    }

    fn traverse_left(&self, stack: &mut NodeStack, collected_string: &mut String, current_node: Option<Node>) {
        let mut finished_traverse = false; 
        let mut head = current_node;
        while finished_traverse != true {
            stack.push(head.clone().unwrap());
            head = match self.left() {
                Some(node) => Some(node),
                None => {finished_traverse = true; head}, 
            };
        }
        *collected_string += head.unwrap().value.as_str(); 
    }
   


    pub fn collect(&self) -> String {
        let mut stack = NodeStack::new(); 
        let mut head = match self.head() {
            None => return String::new(),
            Some(x) => Some(x)
        };
        
        let data: String = String::new();
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

}
