use lazy_static::lazy_static;
use regex::Regex;
use rust::utils;
use std::cell::RefCell;
use std::path::Path;

struct LoadingParams {
    n: u32,
    from: usize,
    to: usize,
}

impl LoadingParams {
    fn new(n: u32, from: usize, to: usize) -> Self {
        LoadingParams { n, from, to }
    }
}

trait Crane {
    fn move_cargo(params: &LoadingParams, state: &RefCell<Vec<Vec<char>>>) -> () {
        let mut cargo: char;

        for _ in 1..=params.n {
            cargo = state.borrow_mut()[params.from].pop().unwrap();
            state.borrow_mut()[params.to].push(cargo);
        }

        // println!("Moving cargo with CM 9000!");
    }
}

struct Cm9000 {}

impl Cm9000 {
    fn new() -> Self {
        Cm9000 {}
    }
}

impl Crane for Cm9000 {}

struct Cm9001 {}

impl Cm9001 {
    fn new() -> Self {
        Cm9001 {}
    }
}

impl Crane for Cm9001 {
    fn move_cargo(params: &LoadingParams, state: &RefCell<Vec<Vec<char>>>) -> () {
        let mut cargo: char;
        let mut cargo_vec: Vec<char> = vec![];

        for _ in 1..=params.n {
            cargo = state.borrow_mut()[params.from].pop().unwrap();
            cargo_vec.push(cargo);
        }

        // println!("Moving cargo with CM 9001!");
        for _ in 1..=params.n {
            state.borrow_mut()[params.to].push(cargo_vec.pop().unwrap());
        }
    }
}

struct LoadingDock<'a, T: Crane> {
    state: RefCell<Vec<Vec<char>>>,
    cargo_mover: T,
    commands: Vec<&'a str>,
}

impl<'a, T: Crane> LoadingDock<'a, T> {
    fn new(cargo_mover: T) -> Self {
        LoadingDock {
            state: RefCell::new(vec![]),
            cargo_mover,
            commands: vec![],
        }
    }

    pub fn parse_init(&mut self, init: &'a str) -> () {
        let split_pos = init.find("\n\n").unwrap();

        let (init_state_str, commands_str) = init.split_at(split_pos);

        // println!("{}<split!>{}", init_state_str, commands_str);

        self.commands = commands_str
            .trim()
            .lines()
            .to_owned()
            .collect::<Vec<&str>>();

        self.parse_state(init_state_str);
    }

    fn parse_state(&self, state_str: &str) -> () {
        lazy_static! {
            static ref POS_RE: Regex = Regex::new("[0-9]+").unwrap();
            static ref CHAR_RE: Regex = Regex::new("[A-Z]").unwrap();
        }

        let mut positions: Vec<usize> = vec![];
        let mut curr_char: char;

        let mut state_vec: Vec<Vec<char>> = vec![];

        // println!("State string is {:?}", state_str);

        for line in state_str.lines().rev() {
            if POS_RE.is_match(line) {
                let matches = POS_RE.find_iter(line);

                // Initialize stack cols.
                for match_res in matches {
                    positions.push(match_res.start());
                    state_vec.push(vec![]);
                }
                // println!("Positions are {:?}", positions);
            } else {
                for (i, &position) in positions.iter().enumerate() {
                    curr_char = line.chars().collect::<Vec<char>>()[position];
                    if curr_char != ' ' {
                        state_vec[i].push(curr_char);
                    }
                }
            }
        }

        // println!("Constructed state as {:?}", state_vec);
        self.state.replace(state_vec.clone());
    }

    fn parse_command(command: &str) -> LoadingParams {
        lazy_static! {
            static ref PARAM_RE: Regex = Regex::new("[0-9]+").unwrap();
        }

        let mut params = PARAM_RE.find_iter(command);

        let n = params.next().unwrap().as_str().parse::<u32>().unwrap();

        let from = params.next().unwrap().as_str().parse::<usize>().unwrap();

        let to = params.next().unwrap().as_str().parse::<usize>().unwrap();

        // Account for 0 indexing.
        LoadingParams::new(n, from - 1, to - 1)
    }

    fn run_commands(&self) -> () {
        for command in &self.commands {
            let params = &LoadingDock::<T>::parse_command(command);
            T::move_cargo(params, &self.state);
        }
    }

    fn report_top_cargo(&self) -> Vec<char> {
        let mut top_cargo: Vec<char> = vec![];
        let state_borrow = self.state.borrow();

        for stack in state_borrow.iter() {
            top_cargo.push(stack[stack.len() - 1]);
        }

        top_cargo
    }
}

fn main() {
    let path = Path::new("../data/day_05.txt");
    let mut dock_9000 = LoadingDock::new(Cm9000::new());
    let mut dock_9001 = LoadingDock::new(Cm9001::new());

    // Create a path to the desired file
    let init = utils::string_from_file(path);
    let init_str: &str = init.as_str();

    dock_9000.parse_init(init_str);
    dock_9000.run_commands();

    dock_9001.parse_init(init_str);
    dock_9001.run_commands();

    let top_cargo_9000 = dock_9000.report_top_cargo();
    let top_cargo_9001 = dock_9001.report_top_cargo();

    println!(
        "Top crates for CM 9000 are {}",
        top_cargo_9000.iter().collect::<String>()
    );
    println!(
        "Top crates for CM 9001 are {}",
        top_cargo_9001.iter().collect::<String>()
    );
}
