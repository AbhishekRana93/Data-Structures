mod tree_node;

mod pre_order;
mod insert;

use std::rc::Rc;
use std::cell::RefCell;
use self::tree_node::TreeNode;

pub fn execute() {

    let mut node = TreeNode::new(15);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));

    let node_rt_lt = Some(Rc::new(RefCell::new(TreeNode::new(17))));

    let mut node_rt = TreeNode::new(21);
    node_rt.left = node_rt_lt;

    node.right = Some(Rc::new(RefCell::new(node_rt)));
    let mut node_option = Some(Rc::new(RefCell::new(node)));

    // node_option = None;
    node_option = insert::insert_into_bst(node_option, 13);

    pre_order::pre_order_traversal(&node_option, &|x| {
        println!("{:?}", x);
    });

}
