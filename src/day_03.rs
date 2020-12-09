use crate::puzzle::Puzzle;

pub fn run() {
  let a = Puzzle {
    name: "03-a",
    solution: solution_a,
  };
  a.run_test("7");
  a.run();

  let b = Puzzle {
    name: "03-b",
    solution: solution_b,
  };
  b.run_test("336");
  b.run();
}

fn parse_line(line: &str) -> Vec<bool> {
  line.chars().map(|c| c == '#').collect()
}

fn run_slope(forest: &Vec<Vec<bool>>, step_x: usize, step_y: usize) -> u32 {
  let forest_width = forest[0].len();
  let mut trees = 0;
  let mut x = 0;

  for y in (0..forest.len()).step_by(step_y) {
    if forest[y][x % forest_width] {
      trees += 1;
    }
    x = (x + step_x) % forest_width;
  }

  trees
}

fn solution_a(input: &String) -> Option<String> {
  let forest: Vec<Vec<bool>> = input.lines().map(parse_line).collect();

  Some(run_slope(&forest, 3, 1).to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let forest: Vec<Vec<bool>> = input.lines().map(parse_line).collect();

  let result: u64 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|(x, y)| run_slope(&forest, *x, *y))
    .fold(1, |acc, t| acc * t as u64);

  Some(result.to_string())
}
