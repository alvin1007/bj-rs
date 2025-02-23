use std::collections::{HashMap, BTreeSet};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    let mut graph = HashMap::new();
    let mut visited = HashMap::new();

    for _ in 0..n-1 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let edge = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

        graph.entry(edge[0]).or_insert_with(BTreeSet::new).insert(edge[1]);
        graph.entry(edge[1]).or_insert_with(BTreeSet::new).insert(edge[0]);

        visited.entry(edge[0]).or_insert(false);
        visited.entry(edge[1]).or_insert(false);
    }

    let mut dp = [[0; 2]; 1000001];

    dfs(1, &graph, &mut visited, &mut dp);

    println!("{}", dp[1][0].min(dp[1][1]));
}

fn dfs(start: i64, graph: &HashMap<i64, BTreeSet<i64>>, visited: &mut HashMap<i64, bool>, dp: &mut [[i32; 2]]) {
    visited.entry(start).and_modify(|x| *x = true );

    let edge = match graph.get(&start) {
        Some(e) => e,
        None => return,
    };

    dp[start as usize][0] = 1;

    for next in edge {
        if !visited.get(next).unwrap() {
            dfs(*next, graph, visited, dp);
            dp[start as usize][1] += dp[*next as usize][0];
            dp[start as usize][0] += dp[*next as usize][0].min(dp[*next as usize][1]);
        }
    }
}