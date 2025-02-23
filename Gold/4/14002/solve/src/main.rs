fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let _ = buf.trim().parse::<i64>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let a: Vec<i64> = buf
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i64>())
        .collect();

    let mut dp: Vec<i64> = Vec::new();
    let mut index = Vec::new();
    let mut parent = vec![None; a.len()];

    for (i, &num) in a.iter().enumerate() {
        let pos = dp.binary_search(&num).unwrap_or_else(|x| x);
        
        if pos == dp.len() {
            dp.push(num);
            index.push(i);
        } else {
            dp[pos] = num;
            index[pos] = i;
        }

        parent[i] = if pos > 0 { Some(index[pos - 1]) } else { None };
    }

    use std::fmt::Write;

    let mut stdout = String::new();
    writeln!(stdout, "{}", dp.len()).unwrap();

    let mut ans = Vec::new();
    let mut pos = index.last().copied();

    while let Some(i) = pos {
        ans.push(a[i]);
        pos = parent[i];
    }

    ans.reverse();
    
    writeln!(stdout, "{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
    print!("{stdout}");
}
