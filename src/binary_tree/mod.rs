mod tree_node;
mod all_possible_fbt;


pub fn partition_labels(s: String) -> Vec<i32> {
    // let n = s.len();
    // let mut vec = vec![0; n];
    // let mut ans;
    //
    // for (i, c) in s.chars().enumerate() {
    //     let val = (c as u32 - 'a' as u32) as usize;
    //
    //     if vec[val] == 0 {
    //         vec[val] = i;
    //     } else {
    //         let prev = vec[val];
    //         for j in prev..i {
    //             // vec[]
    //         }
    //     }
    //
    // }

    return Vec::new();
}

pub fn execute() {
    // println!("{:?}", all_possible_fbt::all_possible_fbt(5));
    println!("{:?}", partition_labels("abcd".to_string()));
}
