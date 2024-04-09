use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            element: x,
            left: None,
            right: None,
        }
    }
}

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        tree.take();
    }
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match tree {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match tree {
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
        None => None,
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match tree {
        Some(node) => {
            if node.right.is_none() {
                Some(node)
            } else {
                find_max(&node.right)
            }
        }
        None => None,
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if tree.is_none() {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        let node = tree.as_mut().unwrap();
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if node.left.is_none() {
                *tree = node.right.take();
            } else if node.right.is_none() {
                *tree = node.left.take();
            } else {
                let mut min_right = node.right.as_mut().unwrap();
                while min_right.left.is_some() {
                    min_right = min_right.left.as_mut().unwrap();
                }
                node.element = min_right.element;
                delete(min_right.element, &mut node.right);
            }
        }
    }
}

fn retrieve(node: &Box<TreeNode>) -> i32 {
    node.element
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }

    for i in 0..50 {
        if find(i, &tree).map_or(false, |node| node.element != i) {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut tree);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if find(i, &tree).map_or(false, |node| node.element != i) {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &tree).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    if let Some(min) = find_min(&tree) {
        println!("Min is {}", retrieve(min));
    }

    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(max));
    }
}