use crate::puzzle::Puzzle;

pub fn run() {
  let a = Puzzle {
    name: "01-a",
    solution: solution_a,
  };
  a.run_test("514579");
  a.run();

  let b = Puzzle {
    name: "01-b",
    solution: solution_b,
  };

  b.run_test("241861950");
  b.run();
}

fn parse_line(line: &String) -> u32 {
  line.parse::<u32>().unwrap()
}

fn solution_a(input: &Vec<String>) -> Option<String> {
  let numbers: Vec<u32> = input.iter().map(parse_line).collect();

  for (i, num_a) in numbers.iter().enumerate() {
    for num_b in numbers.iter().skip(i) {
      if num_a + num_b == 2020 {
        return Some((num_a * num_b).to_string());
      }
    }
  }

  None
}

fn solution_b(input: &Vec<String>) -> Option<String> {
  let numbers: Vec<u32> = input.iter().map(parse_line).collect();

  for (i, num_a) in numbers.iter().enumerate() {
    for (j, num_b) in numbers.iter().enumerate().skip(i) {
      for num_c in numbers.iter().skip(j) {
        if num_a + num_b + num_c == 2020 {
          return Some((num_a * num_b * num_c).to_string());
        }
      }
    }
  }

  None
}
