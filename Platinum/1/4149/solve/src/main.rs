use std::io::Write;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let mut v: Vec<u128> = vec![];
    pr(buffer.trim().parse::<u128>().unwrap(), &mut v);
    v.sort();

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    for i in v {
        writeln!(stdout, "{i}").unwrap();
    }
}

static PRIME: &[u128] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

fn pr(n: u128, v: &mut Vec<u128>) {
    if n == 1 { return; }
    if n % 2 == 0 { v.push(2); pr(n/2, v); return; }
    if is_prime(n) { v.push(n); return; }

    let mut a = n;
    let mut b = n;
    let mut c = n;
    let mut g = n;

    loop {
        if g == n {
            let t = random() % (n - 2) + 2;
            a = t;
            b = t;
            c = random() % 10 + 1;
        }
        a = (c + ((a*a)%n))%n;
        b = (c + ((b*b)%n))%n;
        b = (c + ((b*b)%n))%n;
        g = gcd((a as i128 - b as i128).abs() as u128, n);

        if g != 1 { break; }
    }
    pr(g, v);
    pr(n / g, v);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 { return a; }
    return gcd(b, a%b);
}

fn random() -> u128 {
    let n = vec![0];
    let address = &n as *const Vec<i32>;
    address as u128
}

fn pow(mut a: u128, mut m: u128, p: u128) -> u128 {
    a %= p;
    m %= p;

    let mut r: u128 = 1;
    let mut w: u128 = a;

    while m != 0 {
        if m % 2 == 1 { r = (r * w) % p; }
        w = (w * w) % p;
        m >>= 1;
    }

    return r;
}

fn mr(n: u128, a: u128) -> bool {
    let mut k = n - 1;

    if n % a == 0 { return false; }
    
    loop {
        let d = pow(a, k, n);
        if k % 2 == 1 { return (d != 1) && (d != n - 1); }
        else if d == (n - 1) { return false; }
        k >>= 1;
    }
}

fn is_prime(n: u128) -> bool {
    if n == 1 { return false; }

    for i in PRIME {
        if *i == n { return true; }
        if mr(n, *i) && n > 40 { return false; }
    }

    if n <= 40 { return false; }

    return true;
}