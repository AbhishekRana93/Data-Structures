mod bst;
mod graph;
mod binary_tree;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort();

        let mut ans : Vec<i32> = vec![];

        let mut i = 0;
        let mut j = nums.len();
        while i < j {
            if nums[i] + nums[j] < target {
                i += 1;
            } else if nums[i] + nums[j] > target {
                j -= 1;
            } else {
                ans.push(nums[i]);
                ans.push(nums[j]);
                break;
            }
        }

        return ans;
    }

fn main() {
    // bst::execute();
    // graph::execute();
    // binary_tree::execute();
    print!("{:?}", two_sum(vec![2,7,11,15], 9));
}
