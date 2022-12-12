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

    pub fn is_overlapping_full(&self) -> bool {
        let first_elf = self.first_elf();
        let last_elf = self.last_elf();

        // TODO Fix bug of elf_b first, but a & b are tied for last.
        first_elf == last_elf
    }

    fn first_elf(&self) -> [u32; 2] {
        // First elf is the elf who has to go first (elf_a if both)
        // elves are tied.
        let elf_a = self.extract_interval()[0];
        let elf_b = self.extract_interval()[1];

        if elf_a[0] >= elf_b[0] {
            return elf_a;
        } else {
            return elf_b;
        }
    }

    fn last_elf(&self) -> [u32; 2] {
        // Last elf is the elf who has to go last (elf_a if both)
        // elves are tied.
        let elf_a = self.extract_interval()[0];
        let elf_b = self.extract_interval()[1];

        if elf_a[1] <= elf_b[1] {
            return elf_a;
        } else {
            return elf_b;
        }
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

    for pairing_string in pairing_strings.lines() {
        let pairing = Pairing::new(pairing_string);

        if pairing.is_overlapping_full() {
            // println!("Pairing_string {} overlaps!", pairing_string);
            full_overlap_count += 1;
        } else {
            // println!("Pairing_string {} doesn't overlap!", pairing_string);
        }
    }

    println!("Found {} full overlaps", full_overlap_count);
}
