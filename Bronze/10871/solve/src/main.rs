use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let n = match input_number.split_whitespace().collect::<Vec<&str>>()[0].parse::<usize>() {
        Ok(i) => i,
        Err(_e) => 0,
    };

    let x = input_number.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    {
        let mut input_case = String::new();

        io::stdin().read_line(&mut input_case)
            .expect("Failed to read line");

        let mut i: usize = 0;
        let result: Vec<&str> = input_case.split_whitespace().collect();

        while i < n {
            let m = match result[i].parse::<i32>() {
                Ok(i) => i,
                Err(_e) => -1,
            };
            
            if m < x {
                print!("{} ", m);
            }
            i += 1;
        }
    }
}
