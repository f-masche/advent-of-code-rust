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

fn solution_a(input: &Vec<String>) -> Option<String> {
  let result: usize = groups(input)
    .iter()
    .map(|group| group.iter().flat_map(|line| line.chars()).unique().count())
    .sum();

  Some(result.to_string())
}

fn groups(input: &Vec<String>) -> Vec<Vec<String>> {
  let mut groups = Vec::new();
  let mut current = Vec::new();

  for line in input {
    if line.is_empty() {
      groups.push(current);
      current = Vec::new();
    } else {
      current.push(line.to_string());
    }
  }
  groups.push(current);

  groups
}

fn solution_b(input: &Vec<String>) -> Option<String> {
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
