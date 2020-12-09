use crate::puzzle::Puzzle;
use regex::{Captures, Regex};

lazy_static! {
  static ref PASSWORD_POLICY_PATTERN: Regex =
    Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
}

pub fn run() {
  let a = Puzzle {
    name: "02-a",
    solution: solution_a,
  };
  a.run_test("2");
  a.run();

  let b = Puzzle {
    name: "02-b",
    solution: solution_b,
  };
  b.run_test("1");
  b.run();
}

fn parse_line(line: &str) -> (usize, usize, char, String) {
  PASSWORD_POLICY_PATTERN
    .captures(&line)
    .map(|cap: Captures| {
      let min = cap[1].parse::<usize>().unwrap();
      let max = cap[2].parse::<usize>().unwrap();
      let character = cap[3].chars().next().unwrap();
      let password = cap[4].to_string();
      (min, max, character, password)
    })
    .unwrap()
}

fn solution_a(input: &String) -> Option<String> {
  let count = input
    .lines()
    .map(parse_line)
    .filter(|(min, max, character, password)| {
      let occurences = password.matches(*character).count();
      occurences >= *min && occurences <= *max
    })
    .count()
    .to_string();

  Some(count)
}

fn solution_b(input: &String) -> Option<String> {
  let count = input
    .lines()
    .map(parse_line)
    .filter(|(min, max, character, password)| {
      let first_char = password.chars().nth(min - 1).unwrap();
      let second_char = password.chars().nth(max - 1).unwrap();

      (first_char == *character || second_char == *character) && first_char != second_char
    })
    .count()
    .to_string();

  Some(count)
}
