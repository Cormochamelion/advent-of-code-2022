use std::path::Path;
use rust::utils;
use std::collections::HashSet;

fn find_dupe_chars(str1: &String, str2: &String) -> Vec<char> {
    let comp1_hash: HashSet<char> = str1.chars().collect();
    let comp2_hash: HashSet<char> = str2.chars().collect();

    comp1_hash.intersection(&comp2_hash).cloned().collect::<Vec<char>>()
}

fn char_value(c: &char) -> u32 {
    // FIXME Add check for ASCII
    let c_val = c.clone() as u32;

    if c_val >= 65 && c_val <= 90 {
        // Upercase letters
        return c_val - 64 + 26
    } else if c_val >= 97 && c_val <= 122 {
        // Lowercase letters
        return c_val - 96
    } else {
        panic!("Char {} doesn't seem to be a ASCII letter.", c);
    }
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("../data/day_03.txt");

    let packings = utils::string_from_file(path);

    let mut sum: u32 = 0;

    for packing in packings.lines() {
        let packing_mid = packing.chars().count() / 2;
        let (comp1, comp2) = packing.split_at(packing_mid);
        let dupe_vec = find_dupe_chars(&comp1.to_string(), &comp2.to_string());
        let dupe_vals = dupe_vec.
            iter()
            .map(|x| char_value(x))
            .collect::<Vec<u32>>();

        sum += dupe_vals.iter().sum::<u32>();

        // println!("Compartments {:?} and {:?} contain dupes {:?}", comp1, comp2, dupe_vec);
        //println!("Dupes {:?} have prios {:?}", dupe_vec, dupe_vals);
    } 

    println!("Total sum of priorities is {}", sum);
}