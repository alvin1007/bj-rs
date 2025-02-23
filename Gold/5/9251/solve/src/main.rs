fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s1 = buf.trim().parse::<String>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s2 = buf.trim().parse::<String>().unwrap();

    lcs(s1, s2);
}

fn lcs(a: String, b: String) {
    let mut table = vec![vec![0; a.len() + 1]; b.len() + 1];

    let a_byte = a.as_bytes();
    let b_byte = b.as_bytes();

    for i in 1..b.len()+1 {
        for j in 1..a.len()+1 {
            if a_byte[j-1] != b_byte[i-1] {
                table[i][j] = table[i-1][j].max(table[i][j-1]);
            } else {
                table[i][j] = table[i-1][j-1] + 1;
            }
        }
    }

    println!("{}", table[b.len()][a.len()]);
}