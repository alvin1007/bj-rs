fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let t = next().parse().unwrap();

    let mem = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (1, 2),
        (2, 3),
        (3, 5),
        (5, 8),
        (8, 13),
        (13, 21),
        (21, 34),
        (34, 55),
        (55, 89),
        (89, 144),
        (144, 233),
        (233, 377),
        (377, 610),
        (610, 987),
        (987, 1597),
        (1597, 2584),
        (2584, 4181),
        (4181, 6765),
        (6765, 10946),
        (10946, 17711),
        (17711, 28657),
        (28657, 46368),
        (46368, 75025),
        (75025, 121393),
        (121393, 196418),
        (196418, 317811),
        (317811, 514229),
        (514229, 832040),
        (832040, 1346269),
        (1346269, 2178309),
        (2178309, 3524578),
        (3524578, 5702887),
        (5702887, 9227465),
        (9227465, 14930352),
        (14930352, 24157817),
        (24157817, 39088169),
        (39088169, 63245986),
        (63245986, 102334155),
    ];

    for _ in 0..t {
        let n = next().parse::<usize>().unwrap();
        let res = mem[n];
        writeln!(stdout, "{} {}", res.0, res.1).unwrap();
    }

    print!("{stdout}")
}