use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub fn run() {
  let a = Puzzle {
    name: "10-a",
    solution: solution_a,
  };
  a.run_test("220");
  a.run();

  let b = Puzzle {
    name: "10-b",
    solution: solution_b,
  };
  b.run_test("19208");
  b.run();
}

fn parse_adapters(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|line| line.parse::<u32>().unwrap())
    .collect()
}

fn connect(adapters: &Vec<u32>) -> Vec<u32> {
  let mut result = adapters.clone();
  result.push(0);
  result.sort();
  result.push(result.last().unwrap() + 3);
  result
}

fn get_arrangements(adapters: &Vec<u32>) -> u64 {
  let len = adapters.len();
  let mut counts: Vec<u64> = vec![0; len];
  counts.insert(0, 1);

  for i in 0..len {
    for j in (i + 1)..len {
      if adapters[j] > adapters[i] + 3 {
        break;
      }
      counts[j] += counts[i];
    }
  }
  counts[len - 1]
}

fn solution_a(input: &String) -> Option<String> {
  let adapters = connect(&parse_adapters(input));
  let diffs: HashMap<u32, u32> = adapters
    .windows(2)
    .map(|window| window[1] - window[0])
    .fold(HashMap::new(), |mut acc, x| {
      *acc.entry(x).or_insert(0) += 1;
      acc
    });

  Some((diffs[&1] * diffs[&3]).to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let result = get_arrangements(&connect(&parse_adapters(input)));

  Some(result.to_string())
}
