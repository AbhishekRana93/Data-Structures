mod all_paths_source_target;

pub fn execute() {
    println!("{:?}", all_paths_source_target::all_paths_source_target(
        vec![vec![4,3,1],vec![3,2,4],vec![3],vec![4],vec![]])
    );
}
