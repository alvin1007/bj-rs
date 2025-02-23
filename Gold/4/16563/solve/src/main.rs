fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();

    let mut next = || tokens.next().unwrap();
    let t = next().parse().unwrap();

    let mut nums = vec![0; 5000001];

    for i in 2..=5000000 {
        nums[i] = i;
    }

    for i in 2..=5000000 {
        if nums[i] == i {
            let mut j = i * i;
            while j < 5000001 {
                if nums[j] == j {
                    nums[j] = i;
                }
                j += i;
            }
        }
    }

    for _ in 0..t {
        let mut k: usize = next().parse().unwrap();

        while k > 1 {
            write!(stdout, "{} ", nums[k]).unwrap();
            k /= nums[k];
        }
        write!(stdout, "\n").unwrap();
    }

    print!("{stdout}");
}

