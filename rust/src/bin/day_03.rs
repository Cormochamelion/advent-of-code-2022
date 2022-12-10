use std::path::Path;
use rust::utils;
use itertools::Itertools;
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

fn badge_from_uniques(packing: &Vec<HashSet<char>>) -> char {
    let mut candidate: char = 'a';
    let mut candidate_count: u32 = 1;
    let mut badge: Option<char> = None;

    let mut pack_vec: Vec<char> = vec![];

    for set in packing {
        let mut set_vec = set.clone().into_iter().collect::<Vec<char>>();
        pack_vec.append(&mut set_vec);
    }

    pack_vec = pack_vec.into_iter().sorted().collect::<Vec<char>>();
    
    for item in pack_vec {
        if item == candidate {
            candidate_count += 1;

            if candidate_count >= 3 {
                badge = Some(candidate);
                break;
            }
        } else {
            candidate = item;
            candidate_count = 1;
        }
    };

    badge.unwrap()
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("../data/day_03.txt");

    let packings = utils::string_from_file(path);

    let mut sum_dupe: u32 = 0;
    let mut sum_badge: u32 = 0;
    let mut group_count: u32 = 0;
    let mut group_vec: Vec<HashSet<char>> = vec![];
    let mut group_badge: char;

    for packing in packings.lines() {
        // Find sum of dupe prios.
        let packing_mid = packing.chars().count() / 2;
        let (comp1, comp2) = packing.split_at(packing_mid);
        let dupe_vec = find_dupe_chars(&comp1.to_string(), &comp2.to_string());
        let dupe_vals = dupe_vec.
            iter()
            .map(|x| char_value(x))
            .collect::<Vec<u32>>();

        sum_dupe += dupe_vals.iter().sum::<u32>();
        
        // Find sum of badge prios.
        group_vec.push(packing.chars().collect::<HashSet<char>>());
        group_count += 1;

        if group_count >= 3 {
            group_badge = badge_from_uniques(&group_vec);

            sum_badge += char_value(&group_badge);

            group_count = 0;
            group_vec = vec![];
        }
    } 

    println!("Total sum of dupe priorities is {}", sum_dupe);
    println!("Total sum of badge priorities is {}", sum_badge);
}