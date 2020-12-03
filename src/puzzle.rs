use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
};
use termion::{color, style};

const INPUT_PATH: &str = "data/%id%.txt";

const TEST_INPUT_PATH: &str = "data/%id%-test.txt";

pub struct Puzzle<'a> {
  pub name: &'a str,
  pub solution: fn(&Vec<String>) -> Option<String>,
}

impl Puzzle<'_> {
  pub fn run(&self) {
    let input = Puzzle::get_input(INPUT_PATH.replace("%id%", &self.name));
    let result: Option<String> = (self.solution)(&input);

    match result {
      Some(x) => println!(
        "{}✔︎{} Solution {}: {}\"{}\"\n",
        color::Fg(color::Green),
        style::Reset,
        self.name,
        color::Fg(color::Green),
        x
      ),
      None => println!(
        "{}😡 No solution found for {}\n",
        color::Fg(color::Red),
        self.name
      ),
    };
  }

  pub fn run_test(&self, expected: &str) {
    let test_input = Puzzle::get_input(TEST_INPUT_PATH.replace("%id%", &self.name));
    let test_result = (self.solution)(&test_input);

    match test_result {
      Some(x) => {
        if x == expected.to_string() {
          println!(
            "{}✔︎{} Test {} passed",
            color::Fg(color::Green),
            style::Reset,
            self.name,
          );
        } else {
          println!(
            "{}💥 Test {} failed: \"{}\" != \"{}\"",
            color::Fg(color::Red),
            self.name,
            x,
            expected
          );
        }
      }
      None => println!(
        "{}😡 No test result returned for \"{}\"",
        color::Fg(color::Red),
        self.name
      ),
    };
  }

  fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf
      .lines()
      .map(|l| l.expect("Could not parse line"))
      .filter(|l| l.trim().len() > 0)
      .collect()
  }
}
