use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match &root {
            Some(val) => {
                Self::helper(&root, val.borrow().val)
            },
            None => true
        }
    }

    pub fn helper(root : &Option<Rc<RefCell<TreeNode>>>, k : i32) -> bool {
        match root {
            None => true,
            Some(val) =>
                Self::helper(&val.borrow().left, k) &&
                Self::helper(&val.borrow().right, k) &&
                val.borrow().val == k
        }
    }
}

pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, k : i32) -> bool {
    match root {
        Some(val) => {
            // println!("{:?}", val.borrow());
            val.borrow().val == k && helper(&val.borrow().left, k) && helper(&val.borrow().right, k)
            // true
        },
        None => true
    }
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    fn helper(root : &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        match root {
            None => 0,
            Some(ref_val) => {
                let x = helper(&ref_val.borrow().left, l, r) +
                        helper(&ref_val.borrow().right, l, r);
                let y = ref_val.borrow().val;

                if y >= l && y <= r {
                    return y + x;
                }

                x
            }
        }
    }

    helper(&root, l, r)
}

fn main() {

    // let x = vec![-4,-1,0,3,10];
    let mut node = TreeNode :: new(5);

    node.left = Some(Rc::new(RefCell::new(TreeNode :: new(5))));
    let node_rt_lt = Some(Rc::new(RefCell::new(TreeNode :: new(5))));

    let mut node_rt = TreeNode :: new(5);
    node_rt.left = node_rt_lt;

    node.right = Some(Rc::new(RefCell::new(node_rt)));

    // print!("{:?}\n", Solution::is_unival_tree(Some(Rc::new(RefCell::new(node)))));
    print!("{:?}\n", range_sum_bst(Some(Rc::new(RefCell::new(node))), 1,6));

}
