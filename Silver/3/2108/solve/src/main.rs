fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    let mut nums: Vec<i64> = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse::<i64>().unwrap());
    }

    nums.sort();

    println!("{}", (nums.iter().sum::<i64>() as f64 / nums.len() as f64).round() as i64);
    println!("{}", nums[nums.len()/2]);
    println!("{}", {
        let mut result: Vec<i64> = Vec::new();
        let mut frequencies = std::collections::HashMap::new();
        let mut max = 0;
    
        for i in &nums {
            let count = frequencies.entry(i).or_insert(0);
            *count += 1;
        }
    
        for (_int, frequency) in &frequencies {
            if *frequency > max {
                max = *frequency;
            }
        }
    
        for (int, frequency) in &frequencies {
            if *frequency == max {
                result.push(**int);
            }
        }

        result.sort();
      
        if result.len() != 1 { result[1] } else { result[0] }
    });
    println!("{}", nums[nums.len() - 1] - nums[0]);
}
