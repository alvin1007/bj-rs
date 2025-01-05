use std::io;

fn main() {
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number = match number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    {
        let mut i = 1;

        while i <= number {
            
            let mut j = 0;

            while j < i {
                print!("*");
                j += 1;
            }
            i += 1;

            print!("\n");
        }
    }
}
