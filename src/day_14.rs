use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub fn run() {
  let a = Puzzle {
    name: "14-a",
    solution: solution_a,
  };
  a.run_test("165");
  a.run();

  let b = Puzzle {
    name: "14-b",
    solution: solution_b,
  };
  b.run_test("208");
  b.run();
}

fn parse(input: &str) -> Vec<(String, u64, u64)> {
  input
    .lines()
    .map(|line| {
      return if line.starts_with("mem") {
        let parts = line
          .split(" = ")
          .map(|x| {
            x.replace("mem[", "")
              .replace(']', "")
              .parse::<u64>()
              .unwrap()
          })
          .collect::<Vec<u64>>();
        ("mem".to_string(), parts[0], parts[1])
      } else {
        (line[7..].to_string(), 0, 0)
      };
    })
    .collect()
}

fn build_mask(mask: &str) -> Vec<(char, u64)> {
  mask
    .chars()
    .rev()
    .enumerate()
    .map(|(i, bit)| (bit, 1u64 << i))
    .collect::<Vec<(char, u64)>>()
}

fn solution_a(input: &String) -> Option<String> {
  let program = parse(input);
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let mut mask: Vec<(char, u64)> = vec![];

  for (instruction, address, value) in program {
    if instruction == "mem" {
      memory.insert(
        address,
        mask.iter().fold(value, |acc, (op, x)| {
          if *op == '1' {
            return acc | x;
          } else if *op == '0' {
            return acc & !x;
          }
          acc
        }),
      );
    } else {
      mask = build_mask(&instruction);
    }
  }

  let result: u64 = memory.values().sum();
  Some(result.to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let program = parse(input);
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let mut mask: Vec<(char, u64)> = vec![];

  for (instruction, address, value) in program {
    if instruction == "mem" {
      let mut addresses = vec![address];

      for (op, x) in mask.iter() {
        if *op == '1' {
          addresses[0] = addresses[0] | x;
        }
      }

      for (op, x) in mask.iter() {
        if *op == 'X' {
          for i in 0..addresses.len() {
            addresses[i] = addresses[i] & !x;
            addresses.push(addresses[i] | x);
          }
        }
      }

      for address in addresses {
        memory.insert(address, value);
      }
    } else {
      mask = build_mask(&instruction);
    }
  }

  let result: u64 = memory.values().sum();
  Some(result.to_string())
}
