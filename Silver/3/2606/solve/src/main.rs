use std::collections::{HashMap, BTreeSet, VecDeque};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let cfg = buf.trim().parse::<i32>().unwrap();

    let mut graph = HashMap::new();
    let mut visited = HashMap::new();

    for _ in 0..cfg {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let edge = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

        graph.entry(edge[0]).or_insert_with(BTreeSet::new).insert(edge[1]);
        graph.entry(edge[1]).or_insert_with(BTreeSet::new).insert(edge[0]);

        visited.entry(edge[0]).or_insert(false);
        visited.entry(edge[1]).or_insert(false);
    }

    print!("{}", bfs(1, &graph, &mut visited) - 1);
}

fn bfs(start: i32, graph: &HashMap<i32, BTreeSet<i32>>, visited: &mut HashMap<i32, bool>) -> usize {
    let mut queue = VecDeque::new();

    let mut visit_node = vec![];

    queue.push_back(start);
    visited.entry(start).and_modify(|x| *x = true );

    while !queue.is_empty() {
        let x = queue.pop_front().unwrap();

        visit_node.push(x);

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
    
    return visit_node.len();
}