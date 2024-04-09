use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(t: &mut Option<Box<TreeNode>>) {
    let mut node = t.take();
    while let Some(mut n) = node {
        make_empty(&mut n.left);
        make_empty(&mut n.right);
        node = n.right;
    }
}

fn find(x: i32, t: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(n) = node {
        if x < n.element {
            node = n.left.as_ref();
        } else if x > n.element {
            node = n.right.as_ref();
        } else {
            return Some(n);
        }
    }
    None
}

fn find_min(t: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(n) = node {
        if n.left.is_none() {
            return Some(n);
        }
        node = n.left.as_ref();
    }
    None
}

fn find_max(t: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(n) = node {
        if n.right.is_none() {
            return Some(n);
        }
        node = n.right.as_ref();
    }
    None
}

fn insert(x: i32, t: &mut Option<Box<TreeNode>>) {
    if t.is_none() {
        *t = Some(Box::new(TreeNode::new(x)));
    } else {
        let mut node = t.as_mut().unwrap();
        if x < node.element {
            insert(x, &mut node.left);
        } else {
            insert(x, &mut node.right);
        }
    }
}

fn delete(x: i32, t: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = t.take() {
        if x < node.element {
            delete(x, &mut node.left);
            *t = Some(node);
        } else if x > node.element {
            delete(x, &mut node.right);
            *t = Some(node);
        } else {
            if node.left.is_none() {
                *t = node.right;
            } else if node.right.is_none() {
                *t = node.left;
            } else {
                let mut min_node = node.right.take().unwrap();
                node.element = min_node.element;
                delete(node.element, &mut node.right);
                *t = Some(node);
            }
        }
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn main() {
    let mut t: Option<Box<TreeNode>> = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            assert_eq!(retrieve(p), i);
        } else {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(p) = find(i, &t) {
                assert_eq!(retrieve(p), i);
            } else {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &t).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}", retrieve(min));
    }
    if let Some(max) = find_max(&t) {
        println!("Max is {}", retrieve(max));
    }
}