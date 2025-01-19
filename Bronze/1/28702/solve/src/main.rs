fn main() {
    let mut res;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    res = match buffer.trim().parse::<i64>() {
        Ok(i) => i + 3,
        Err(_) => 0,
    };

    if res != 0 {
        if res % 3 == 0 {
            if res % 5 == 0 {
                println!("FizzBuzz")
            } else {
                println!("Fizz")
            }
        } else {
            if res % 5 == 0 {
                println!("Buzz")
            } else {
                println!("{}", res)
            }
        }

        return;
    }

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    res = match buffer.trim().parse::<i64>() {
        Ok(i) => i + 2,
        Err(_) => 0,
    };

    if res != 0 {
        if res % 3 == 0 {
            if res % 5 == 0 {
                println!("FizzBuzz")
            } else {
                println!("Fizz")
            }
        } else {
            if res % 5 == 0 {
                println!("Buzz")
            } else {
                println!("{}", res)
            }
        }

        return;
    }

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    res = match buffer.trim().parse::<i64>() {
        Ok(i) => i + 1,
        Err(_) => 0,
    };

    if res != 0 {
        if res % 3 == 0 {
            if res % 5 == 0 {
                println!("FizzBuzz")
            } else {
                println!("Fizz")
            }
        } else {
            if res % 5 == 0 {
                println!("Buzz")
            } else {
                println!("{}", res)
            }
        }

        return;
    }
}
