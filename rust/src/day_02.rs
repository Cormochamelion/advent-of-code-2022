use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

fn play_score(p1: &Choice, p2: &Choice) -> u32 {
    let mut hierarchy = vec![&Choice::Paper, &Choice::Rock, &Choice::Scissors];

    if p1 == hierarchy[0] {
        hierarchy.rotate_right(1);
    } else if p1 == hierarchy[2] {
        hierarchy.rotate_left(1);
    }

    // Check if we are behind (loss) or ahead (win) in the hierarchy.
    if p2 == hierarchy[2] {
        return 0;
    } else if p2 == hierarchy[0] {
        return 6;
    } else {
        return 3;
    }
}

fn choice_score(p1: &Choice) -> u32 {
    match p1 {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn match_outcome(p1: &Choice, p2: &Choice) -> u32 {
    choice_score(p2) + play_score(p1, p2)
}

fn parse_strat(strat_str: &str) -> String {
    let mut hierarchy = vec!["B", "A", "C"];
    let instruct_vec: Vec<_> = strat_str.clone().split(' ').collect();

    if instruct_vec[0] == hierarchy[0] {
        hierarchy.rotate_right(1);
    } else if instruct_vec[0] == hierarchy[2] {
        hierarchy.rotate_left(1);
    }

    let mut p2_play = hierarchy[1];

    if instruct_vec[1] == "X" {
        p2_play = hierarchy[2];
    } else if instruct_vec[1] == "Z" {
        p2_play = hierarchy[0];
    }

    strat_str.replace(instruct_vec[1], p2_play)
}

fn parse_choice(choice_str: &str) -> Choice {
    if choice_str == "A" || choice_str == "X" {
        return Choice::Rock;
    } else if choice_str == "B" || choice_str == "Y" {
        return Choice::Paper;
    } else if choice_str == "C" || choice_str == "Z" {
        return Choice::Scissors;
    } else {
        panic!("Can't find a Choice for input {}.", choice_str)
    }
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("../data/day_02.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut plays = String::new();
    match file.read_to_string(&mut plays) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(s) => s,
    };

    let mut score_1: u32 = 0;
    let mut score_2: u32 = 0;

    for play in plays.lines() {
        let play_vec: Vec<Choice> = play.split(' ').map(parse_choice).collect();
        let outcome_1 = match_outcome(&play_vec[0], &play_vec[1]);
        score_1 += outcome_1;

        let strat_vec: Vec<Choice> = parse_strat(play).split(' ').map(parse_choice).collect();
        let outcome_2 = match_outcome(&strat_vec[0], &strat_vec[1]);
        score_2 += outcome_2;
        
        // println!("Input: {:?}; Play {:?}; Strat {:?}", play, play_vec, strat_vec);
        // println!("Score a: {score_1}; Score b: {score_2}");
    }

    println!("Final score (pt 1): {score_1}");
    println!("Final score (pt 2): {score_2}");
}
