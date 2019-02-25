pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut ans = Vec::new();
    let mut path = Vec::new();

    dfs(0, (n-1) as i32, &mut graph.clone(), path, &mut ans);

    return ans;
}

fn dfs(curr: i32, target : i32, graph: &mut Vec<Vec<i32>>, mut path: Vec<i32>,
    ans : &mut Vec<Vec<i32>>) {
    path.push(curr);
    if curr == target {
        ans.push(path.to_vec());
        return;
    }

    for child in graph.clone()[curr as usize].iter() {
        dfs(*child, target, graph, path.clone(), ans);
    }

}
