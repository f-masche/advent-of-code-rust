use crate::puzzle::Puzzle;
use std::collections::HashSet;

pub fn run() {
  let a = Puzzle {
    name: "08-a",
    solution: solution_a,
  };
  a.run_test("5");
  a.run();

  let b = Puzzle {
    name: "08-b",
    solution: solution_b,
  };
  b.run_test("8");
  b.run();
}

fn parse_code(code: &str) -> Vec<(&str, i32)> {
  code
    .lines()
    .map(|line| {
      let parts: Vec<&str> = line.split(' ').collect();
      (parts[0], parts[1].parse::<i32>().unwrap())
    })
    .collect()
}

fn exec(code: &Vec<(&str, i32)>) -> Result<i32, i32> {
  let mut pointer: i32 = 0;
  let mut visited: HashSet<i32> = HashSet::new();
  let mut acc: i32 = 0;
  let mut success = false;

  while !success && !visited.contains(&pointer) {
    visited.insert(pointer);
    success = pointer == (code.len() - 1) as i32;

    match code[pointer as usize] {
      ("acc", arg) => {
        acc += arg;
        pointer += 1;
      }
      ("jmp", arg) => pointer += arg,
      ("nop", _) => pointer += 1,
      _ => (),
    }
  }

  return if success { Ok(acc) } else { Err(acc) };
}

fn solution_a(input: &String) -> Option<String> {
  let code = parse_code(input);
  let result = exec(&code);
  Some(result.err().unwrap().to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let code = parse_code(input);
  let mut result: Result<i32, i32> = Err(0);
  let mut pointer: usize = 0;

  while result.is_err() {
    let mut new_code = code.clone();

    for i in pointer..(code.len() - 1) {
      let (op, arg) = code[i];

      if op == "jmp" {
        new_code[i] = ("nop", arg);
        pointer = i + 1;
        break;
      } else if op == "nop" {
        new_code[i] = ("jmp", arg);
        pointer = i + 1;
        break;
      }
    }
    result = exec(&new_code);
  }

  Some(result.ok().unwrap().to_string())
}
