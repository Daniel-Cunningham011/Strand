
trait TreeComponent {
    fn left(&self) -> Option<impl TreeComponent>;
    fn right(&self) -> Option<impl TreeComponent>;
    fn is_leaf(&self) -> bool;
}

#[derive(Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: String,
    size: usize,
}

impl TreeComponent for Node {
    fn left(&self) -> Option<impl TreeComponent> {
        if let Some(x) = &self.left {
            return Some(*(x.clone()));
        }
        return None
    }

    fn right(&self) -> Option<impl TreeComponent> {
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

impl TreeComponent for Rope {
    fn left(&self) -> Option<impl TreeComponent> {
        if let Some(x) = &self.head {
            if let Some(y) = x.left() {
                return Some(y);
            };
        }
        None
    }

    fn right(&self) -> Option<impl TreeComponent> {
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
