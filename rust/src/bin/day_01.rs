use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
     // Create a path to the desired file
    let path = Path::new("../data/day_01.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut calories = String::new();
    match file.read_to_string(&mut calories) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(s) => s,
    };

    let mut sum: u32 = 0;
    let mut sum_vec: Vec<u32> = Vec::new();

    for line in calories.lines() {
        if line == "" {
            sum_vec.push(sum);
            sum = 0;
            continue;
        }

        sum += match line.parse::<u32>() {
            Err(why) => panic!("Couldn't parse {} to type u32: {}", line, why),
            Ok(calorie) => calorie,
        };
    }
    

    let sum_vec_len = sum_vec.len();
    sum_vec.sort();

    println!("Maximum calories: {}", sum_vec[sum_vec_len - 1]);

    
    let top_three_sum: u32 = sum_vec[(sum_vec_len - 3)..=(sum_vec_len - 1)].iter().sum();
    println!("Sum of top three: {}", top_three_sum);
}

