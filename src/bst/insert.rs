use std::rc::Rc;
use std::cell::RefCell;
use super::tree_node::TreeNode;

pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) ->
    Option<Rc<RefCell<TreeNode>>> {

        fn helper(root : &Option<Rc<RefCell<TreeNode>>>, val : i32) ->
        Option<Rc<RefCell<TreeNode>>> {
            match root {
                None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(node) => {
                    if val <= node.borrow().val {
                        let lnode = helper(&node.borrow().left, val);
                        node.borrow_mut().left = lnode;
                    } else {
                        let rnode = helper(&node.borrow().right, val);
                        node.borrow_mut().right = rnode;
                    }

                    return Some(node.clone());
                }
            }
        }

        helper(&root, val)
    }
