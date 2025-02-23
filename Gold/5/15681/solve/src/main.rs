use std::collections::{HashMap, BTreeSet};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let cfg = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

    let mut graph = HashMap::new();
    let mut visited = HashMap::new();

    for _ in 0..cfg[0]-1 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let edge = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

        graph.entry(edge[0]).or_insert_with(BTreeSet::new).insert(edge[1]);
        graph.entry(edge[1]).or_insert_with(BTreeSet::new).insert(edge[0]);

        visited.entry(edge[0]).or_insert(false);
        visited.entry(edge[1]).or_insert(false);
    }
    
    let mut tree = HashMap::new();
    dfs(cfg[1], &graph, &mut visited.clone(), &mut tree);

    use std::fmt::Write;

    let mut stdout = String::new();

    for _ in 0..cfg[2] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i32>().unwrap();
        writeln!(stdout, "{}", tree.get(&n).unwrap()).unwrap();
    }

    print!("{stdout}")
}

fn dfs(start: i32, graph: &HashMap<i32, BTreeSet<i32>>, visited: &mut HashMap<i32, bool>, tree: &mut HashMap<i32, i32>) {
    visited.entry(start).and_modify(|x| *x = true );

    let edge = match graph.get(&start) {
        Some(e) => e,
        None => return,
    };

    tree.entry(start).or_insert(1);

    for next in edge {
        if !visited.get(next).unwrap() {
            dfs(*next, graph, visited, tree);
            tree.insert(start, *tree.get(&next).unwrap() + *tree.get(&start).unwrap());
        }
    }
}