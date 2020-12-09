use crate::puzzle::Puzzle;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
  let a = Puzzle {
    name: "06-a",
    solution: solution_a,
  };
  a.run_test("11");
  a.run();

  let b = Puzzle {
    name: "06-b",
    solution: solution_b,
  };
  b.run_test("6");
  b.run();
}

fn solution_a(input: &String) -> Option<String> {
  let result: usize = groups(input)
    .iter()
    .map(|group| group.iter().flat_map(|line| line.chars()).unique().count())
    .sum();

  Some(result.to_string())
}

fn groups(input: &str) -> Vec<Vec<&str>> {
  let mut groups = Vec::new();

  for group in input.split("\n\n") {
    groups.push(group.lines().collect::<Vec<&str>>());
  }

  groups
}

fn solution_b(input: &String) -> Option<String> {
  let result: usize = groups(input)
    .iter()
    .map(|group| {
      let mut answers: HashMap<char, u8> = HashMap::new();
      let count = group.len() as u8;

      for line in group {
        for c in line.chars() {
          let num = answers.entry(c).or_insert(0);
          *num += 1;
        }
      }
      answers.values().filter(|&num| *num == count).count()
    })
    .sum();

  Some(result.to_string())
}
