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

struct LoadingDock<'a> {
    state: RefCell<Vec<Vec<char>>>,
    commands: Vec<&'a str>,
}

impl<'a> LoadingDock<'a> {
    fn new() -> Self {
        LoadingDock {
            state: RefCell::new(vec![]),
            commands: vec![],
        }
    }

    pub fn move_cargo(&self, params: &LoadingParams) -> () {
        for _ in 1..=params.n {
            let cargo = self.state.borrow_mut()[params.from].pop().unwrap();
            self.state.borrow_mut()[params.to].push(cargo);
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
        let mut stack: Vec<char>;
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
        self.state.replace(state_vec);
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
            let params = &LoadingDock::parse_command(command);
            self.move_cargo(params);
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
    let mut dock = LoadingDock::new();

    // Create a path to the desired file
    let init = utils::string_from_file(path);
    let init_str: &str = init.as_str();

    dock.parse_init(init_str);
    dock.run_commands();
    let top_cargo = dock.report_top_cargo();

    println!("Top crates are {}", top_cargo.iter().collect::<String>());
}
