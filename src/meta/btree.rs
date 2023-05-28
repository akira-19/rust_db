use serde::{Deserialize, Serialize};

const MAX_DEGREE: usize = 3;

#[derive(Serialize, Deserialize)]
struct BTree<T: Item> {
    top: Option<Node<T>>,
    length: usize,
}

#[derive(Serialize, Deserialize)]
struct Node<T: Item> {
    items: Vec<T>,
    children: Vec<Node<T>>,
}

pub type Items<T> = Vec<T>;
pub type IntItem = i32;

pub trait Item: PartialEq + PartialOrd {
    fn less(&self, other: &Self) -> bool;
}

impl Item for IntItem {
    fn less(&self, other: &Self) -> bool {
        *self < *other
    }
}

impl<T: Item> BTree<T> {
    pub fn find(&self, item: &T) -> (bool, usize) {
        if let Some(top) = &self.top {
            top.items.find(item)
        } else {
            (false, 0)
        }
    }

    pub fn insert(&mut self, item: T) {
        if let Some(top) = &mut self.top {
            let (found, index) = top.items.find(&item);
            if !found {
                top.items.insert_at(index, item);
                self.length += 1;
                if top.items.len() > MAX_DEGREE {
                    self.split(&mut top);
                }
            }
        } else {
            self.top = Some(Node {
                items: vec![Item::new(item)],
                children: Vec::new(),
            });
            self.length = 1;
        }
    }

    fn split(&mut self, node: &mut Node<T>) {
        let mid = node.items.len() / 2;
        let mut left = node.items[..mid].to_vec();
        let item = node.items.remove(mid);
        let right = node.items.split_off(mid + 1);

        let mut new_node = Node {
            items: vec![item],
            children: Vec::new(),
        };

        if !node.children.is_empty() {
            let mid_child = node.children.len() / 2;
            let mut left_children = node.children.drain(..mid_child).collect();
            let mut right_children = node.children.drain(..).collect();
            let child = right_children.remove(0);

            new_node.children.push(Node {
                items: right,
                children: right_children,
            });

            node.items = left;
            node.children = left_children;

            if let Some(top) = &mut self.top {
                if top.items.len() < MAX_DEGREE {
                    top.items.insert_at(top.items.len(), new_node);
                } else {
                    let mut new_top = Node {
                        items: Vec::new(),
                        children: vec![top.clone(), new_node],
                    };
                    self.top = Some(new_top);
                }
            }
        } else {
            node.items = left;
            node.children.push(new_node);
        }
    }
}

impl<T: Item> Node<T> {
    fn new() -> Node<T> {
        Node {
            items: Vec::new(),
            children: Vec::new(),
        }
    }
}
