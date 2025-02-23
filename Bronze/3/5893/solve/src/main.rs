fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut s = buf.trim().as_bytes().to_vec();
    let mut k = s.clone();
    k.push(b'0');
    k.push(b'0');
    k.push(b'0');
    k.push(b'0');

    s.reverse();
    k.reverse();

    for i in 0..k.len() {
        if i == s.len() {
            s.push(k[i]);
        } else {
            if k[i] == b'1' {
                let mut t = i;
                while s[t] == b'1' {
                    s[t] = b'0';
                    t += 1;
                    if t == s.len() { s.push(b'1'); break; }
                }
                s[t] = b'1';
            }
        }
    }

    use std::fmt::Write;
    let mut stdout = String::new();

    s.reverse();

    writeln!(stdout, "{}", String::from_utf8(s).unwrap()).unwrap();

    print!("{stdout}")
}