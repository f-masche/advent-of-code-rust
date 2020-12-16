use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub fn run() {
  let a = Puzzle {
    name: "15-a",
    solution: solution_a,
  };
  a.run_test("436");
  a.run();

  let b = Puzzle {
    name: "15-b",
    solution: solution_b,
  };
  b.run_test("175594");
  b.run();
}

fn parse(input: &str) -> Vec<u32> {
  input
    .split(',')
    .map(|x| x.trim().parse::<u32>().unwrap())
    .collect()
}

fn nth_spoken_number(numbers: Vec<u32>, n: usize) -> u32 {
  let mut last = 0;
  let mut occurences: HashMap<u32, (usize, usize)> = HashMap::new();

  for (i, value) in numbers.iter().enumerate() {
    occurences.entry(*value).or_insert((i + 1, i + 1));
    last = *value;
  }

  for step in (numbers.len())..n {
    if let Some(occurence) = occurences.get_mut(&last) {
      last = (step - occurence.1) as u32;
      occurence.0 = occurence.1;
      occurence.1 = step;
    } else {
      occurences.insert(last as u32, (step, step));
      last = 0;
    }
  }

  last
}

fn solution_a(input: &String) -> Option<String> {
  let numbers = parse(&input);
  Some(nth_spoken_number(numbers, 2020).to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let numbers = parse(&input);
  Some(nth_spoken_number(numbers, 30000000).to_string())
}
