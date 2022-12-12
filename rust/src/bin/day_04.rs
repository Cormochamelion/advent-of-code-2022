use std::path::Path;
use rust::utils;

fn main() {
    // Create a path to the desired file
    let path = Path::new("temp.txt");
    let pairings = utils::string_from_file(path);


}