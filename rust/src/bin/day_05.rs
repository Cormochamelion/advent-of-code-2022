use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use rust::utils;

struct LoadingDock<'a> {
    state: Vec<Vec<char>>,
    commands: Vec<&'a str>,
}

impl<'a> LoadingDock<'a> {
    fn new() -> Self {
        LoadingDock { state: vec!(), commands: vec!() }
    }

    pub fn move_cargo(&mut self, n: u32, from: usize, to: usize) -> () {
        for _ in 1..=n {
            let cargo = self.state[from].pop().unwrap();
            self.state[to].push(cargo);
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

    fn parse_state(&mut self, state_str: &str) -> () {

    }

    fn parse_command(&mut self, command: &str) -> (u32, usize, usize) {
        lazy_static! {
            static ref PARAM_RE: Regex = Regex::new("[0-9]+").unwrap();
        }

        let mut params = PARAM_RE.find_iter(command);

        let n = params.next()
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let from = params.next()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let to = params.next()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        (n, from , to)
    }

    fn run_commands(&mut self) -> () {
        for command in self.commands {

        }
    }
}

fn main() {
    // let path = Path::new("../data/day_05.txt");
    let path = Path::new("temp.txt");
    let mut dock = LoadingDock::new();

    // Create a path to the desired file
    let init = utils::string_from_file(path);
    let init_str: &str = init.as_str();

    dock.parse_init(init_str);
     
}