use std::borrow::BorrowMut;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Debug)]
pub enum Node<'a> {
    Val(u32, &'a Node<'a>),
    Nil
}

#[derive(Debug)]
pub struct DynNode {
    val: u32,
    next: RefCell<Weak<DynNode>>
}

fn main() {

    let nodes = vec![Node::Val(1, &Node::Nil), Node::Val(2, &Node::Nil)];
    let mut dyn_nodes = DynNode {val: 1, next: RefCell::new(Weak::new())};
    println!("nodes: {:#?}", nodes);
    println!("dyn_nodes: {:#?}", dyn_nodes);
    *dyn_nodes.next.borrow_mut() = Weak::new();
    dyn_nodes.val = 90;
}
