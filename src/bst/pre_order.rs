use std::rc::Rc;
use std::cell::RefCell;
use super::tree_node::TreeNode;

pub fn pre_order_traversal<F>(root : &Option<Rc<RefCell<TreeNode>>>, f : &F)
    where F: Fn(i32) -> () {
    helper(root, f);

    fn helper<F>(root : &Option<Rc<RefCell<TreeNode>>>, f : &F)
    where F: Fn(i32){
        match root {
            Some(node) => {
                f(node.borrow().val);
                helper(&node.borrow().left, f);
                helper(&node.borrow().right, f);
            },
            None => return,
        }
    }
}
