use std::collections::{HashMap, BTreeSet, VecDeque};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let cfg = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

    let mut graph = HashMap::new();
    let mut visited = HashMap::new();

    for _ in 0..cfg[1] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let edge = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

        graph.entry(edge[0]).or_insert_with(BTreeSet::new).insert(edge[1]);
        graph.entry(edge[1]).or_insert_with(BTreeSet::new).insert(edge[0]);

        visited.entry(edge[0]).or_insert(false);
        visited.entry(edge[1]).or_insert(false);
    }

    dfs(cfg[2], &graph, &mut visited.clone());

    print!("\n");

    bfs(cfg[2], &graph, &mut visited.clone());

    print!("\n");
}

fn dfs(start: i32, graph: &HashMap<i32, BTreeSet<i32>>, visited: &mut HashMap<i32, bool>) {
    visited.entry(start).and_modify(|x| *x = true );

    print!("{start} ");

    let edge = match graph.get(&start) {
        Some(e) => e,
        None => return,
    };

    for next in edge {
        if !visited.get(next).unwrap() {
            dfs(*next, graph, visited);
        }
    }
}

fn bfs(start: i32, graph: &HashMap<i32, BTreeSet<i32>>, visited: &mut HashMap<i32, bool>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.entry(start).and_modify(|x| *x = true );

    while !queue.is_empty() {
        let x = queue.pop_front().unwrap();

        print!("{} ", x);

        let edge = match graph.get(&x) {
            Some(e) => e,
            None => continue,
        };

        for next in edge {
            if !visited.get(next).unwrap() {
                queue.push_back(*next);
                visited.entry(*next).and_modify(|x| *x = true );
            }
        }
    }
}