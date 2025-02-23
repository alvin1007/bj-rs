fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cfg = buffer.split_ascii_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut hs1: std::collections::HashSet<String> = Default::default();
    let mut hs2: std::collections::HashSet<String> = Default::default();

    for _ in 0..cfg[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        hs1.insert(buffer.trim().to_string());
    }

    for _ in 0..cfg[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        hs2.insert(buffer.trim().to_string());
    }

    let mut res = hs1.intersection(&hs2).cloned().collect::<Vec<String>>();
    res.sort();

    use std::fmt::Write;
    let mut stdout = String::new();

    writeln!(stdout, "{}", res.len()).unwrap();

    for s in res {
        writeln!(stdout, "{s}").unwrap();
    }

    print!("{stdout}");
}
