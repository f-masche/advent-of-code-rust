use crate::puzzle::Puzzle;

pub fn run() {
  let a = Puzzle {
    name: "05-a",
    solution: solution_a,
  };
  a.run_test("357");
  a.run();

  let b = Puzzle {
    name: "05-b",
    solution: solution_b,
  };
  b.run();
}

fn get_seat(boarding_pass: &str) -> u32 {
  let mut upper: f32 = 127.0;
  let mut lower: f32 = 0.0;
  let mut upper_row: f32 = 7.0;
  let mut lower_row: f32 = 0.0;

  for i in 0..7 {
    let x = boarding_pass.chars().nth(i).unwrap();

    if x == 'B' {
      lower += ((upper - lower) / 2.0).ceil();
    } else if x == 'F' {
      upper -= ((upper - lower) / 2.0).ceil();
    }
  }

  for i in 7..10 {
    let x = boarding_pass.chars().nth(i).unwrap();

    if x == 'R' {
      lower_row += ((upper_row - lower_row) / 2.0).ceil();
    } else if x == 'L' {
      upper_row -= ((upper_row - lower_row) / 2.0).ceil();
    }
  }

  (lower * 8.0 + lower_row) as u32
}

fn solution_a(input: &String) -> Option<String> {
  let result = input.lines().map(get_seat).max().unwrap();
  Some(result.to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let mut seats: Vec<u32> = input.lines().map(get_seat).collect::<Vec<u32>>();
  seats.sort();

  for (i, seat) in seats.iter().enumerate() {
    if i > 0 && seats[i + 1] == seat + 2 {
      return Some((seat + 1).to_string());
    }
  }

  None
}
