use crate::puzzle::Puzzle;
use std::cmp;

pub fn run() {
  let a = Puzzle {
    name: "09-a",
    solution: solution_a,
  };
  a.run_test("127");
  a.run();

  let b = Puzzle {
    name: "09-b",
    solution: solution_b,
  };
  b.run_test("62");
  b.run();
}

fn parse(input: &str) -> Vec<u64> {
  input
    .lines()
    .map(|line| line.parse::<u64>().unwrap())
    .collect()
}

fn find_wrong_number(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
  'outer: for pointer in preamble..numbers.len() {
    let num = numbers[pointer];

    for a in (pointer - preamble)..(pointer) {
      for b in (pointer - preamble)..(pointer) {
        let num_a = numbers[a];
        let num_b = numbers[b];

        if num_a != num_b && num_a + num_b == num {
          continue 'outer;
        }
      }
    }
    return Some(num);
  }

  None
}

fn find_weakness(numbers: &Vec<u64>, wrong_number: &u64) -> Option<u64> {
  for i in 0..numbers.len() {
    let mut sum = 0;
    let mut min = u64::MAX;
    let mut max = 0;

    for number in numbers.iter().skip(i) {
      sum += number;
      max = cmp::max(*number, max);
      min = cmp::min(*number, min);

      if sum > *wrong_number {
        break;
      } else if sum == *wrong_number {
        return Some(min + max);
      }
    }
  }
  None
}

fn solution_a(input: &String) -> Option<String> {
  let numbers = parse(input);
  let preamble = if numbers.len() <= 20 { 5 } else { 25 };

  find_wrong_number(&numbers, preamble).map(|num| num.to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let numbers = parse(input);
  let preamble = if numbers.len() <= 20 { 5 } else { 25 };

  find_wrong_number(&numbers, preamble)
    .and_then(|wrong_number| find_weakness(&numbers, &wrong_number))
    .map(|num| num.to_string())
}
