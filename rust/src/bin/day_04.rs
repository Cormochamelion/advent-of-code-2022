use rust::utils;
use std::convert::TryInto;
use std::path::Path;

struct Pairing<'a> {
    string: &'a str,
}

impl<'a> Pairing<'a> {
    fn new(string: &'a str) -> Self {
        Pairing { string }
    }

    fn is_overlapping_part(&self) -> bool {
        // Intention
        // >a---|a >b---|b := false
        // >a-->b---|a--|b := true
        // >b-->a---|b--|a := true
        // >b---|b >a---|b := false
        // >a-->b---|b--|a := true
        // >b-->a---|a--|b := true
        let area_interval = self.extract_interval();

        let start_a = area_interval[0][0];
        let start_b = area_interval[1][0];

        let stop_a = area_interval[0][1];
        let stop_b = area_interval[1][1];

        // Case a first
        (start_a <= start_b && start_b <= stop_a)

            // Case b first
            || (start_b <= start_a && start_a <= stop_b)
    }

    fn is_overlapping_full(&self) -> bool {
        // Intention
        // >a---|a >b---|b := false -> given from is_overlapping_part
        // >a-->b---|a--|b := false
        // >b-->a---|b--|a := false
        // >b---|b >a---|a := false -> given from is_overlapping_part
        // >a-->b---|b--|a := true
        // >b-->a---|a--|b := true
        let area_interval = self.extract_interval();

        let start_a = area_interval[0][0];
        let start_b = area_interval[1][0];

        let stop_a = area_interval[0][1];
        let stop_b = area_interval[1][1];

        // Case a first
        (start_a <= start_b && stop_b <= stop_a)

            // Case b first
            || (start_b <= start_a && stop_a <= stop_b)
    }

    fn slice_to_interval(&'a self, arr_slice: &'a [u32]) -> &'a [u32; 2] {
        arr_slice.try_into().unwrap()
    }

    // Should return [[u32; 2]; 2]
    fn extract_interval(&self) -> [[u32; 2]; 2] {
        // Each array represents an elf
        // In each array 0 is start, 1 is stop.
        let assignments = &self
            .string
            .split(",")
            .map(|x| x.split("-").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
            .iter()
            .map(|y| {
                y.iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let interval_a = self.slice_to_interval(&assignments[0][0..=1]);
        let interval_b = self.slice_to_interval(&assignments[1][0..=1]);

        [interval_a.clone(), interval_b.clone()]
    }
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("../data/day_04.txt");
    let pairing_strings = utils::string_from_file(path);

    let mut full_overlap_count: u32 = 0;
    let mut part_overlap_count: u32 = 0;

    for pairing_string in pairing_strings.lines() {
        let pairing = Pairing::new(pairing_string);

        if pairing.is_overlapping_part() {
            part_overlap_count += 1;

            if pairing.is_overlapping_full() {
                println!("Pairing_string {} overlaps fully!", pairing_string);
                full_overlap_count += 1;
            } else {
                println!("Pairing_string {} overlaps partially!", pairing_string);
            }
        } else {            
            println!("Pairing_string {} doesn't overlap!", pairing_string);
        }
         
    }

    println!("Found {} full overlaps", full_overlap_count);
    println!("Found {} partial overlaps", part_overlap_count);
}
