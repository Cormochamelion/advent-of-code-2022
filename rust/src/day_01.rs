use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
     // Create a path to the desired file
    let path = Path::new("../data/day_1.txt");
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
    let mut best_sum: u32 = 0;

    for line in calories.lines() {
        if line == "" {
            if sum > best_sum {
                best_sum = sum;
            }
            sum = 0;
            continue;
        }

        // println!("{}", line);

        sum += match line.parse::<u32>() {
            Err(why) => panic!("Couldn't parse {} to type u32: {}", line, why),
            Ok(calorie) => calorie,
        };
    }

    println!("Maximum calories: {best_sum}")
}

