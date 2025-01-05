use std::io;

fn main() {
    let alphabet = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    let mut string = String::new();

    io::stdin().read_line(&mut string)
        .expect("Failed to read line");

    for i in alphabet {
        let mut cnt = 0;

        for j in string.chars() {
            if i == j { print!("{}", cnt); break;}
            cnt += 1;
        }

        if cnt == string.len() {
            print!("-1")
        }

        print!(" ")
    }
}
