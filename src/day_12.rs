use crate::puzzle::Puzzle;

#[derive(Debug)]
struct Ship {
  north: i32,
  east: i32,
  direction: char,
}

#[derive(Debug)]
struct Waypoint {
  north: i32,
  east: i32,
}

pub fn run() {
  let a = Puzzle {
    name: "12-a",
    solution: solution_a,
  };
  a.run_test("25");
  a.run();

  let b = Puzzle {
    name: "12-b",
    solution: solution_b,
  };
  b.run_test("286");
  b.run();
}

fn parse(input: &str) -> Vec<(char, i32)> {
  input
    .lines()
    .map(|line| {
      (
        line.chars().nth(0).unwrap(),
        line[1..].parse::<i32>().unwrap(),
      )
    })
    .collect()
}

fn turn(direction: char, deg: i32) -> char {
  let dirs = ['N', 'E', 'S', 'W'];
  let i = dirs.iter().position(|&x| x == direction).unwrap();
  dirs[(i + deg as usize / 90) % 4]
}

fn solution_a(input: &String) -> Option<String> {
  let instructions = parse(input);
  let mut ship = Ship {
    north: 0,
    east: 0,
    direction: 'E',
  };

  for (mut instruction, value) in instructions {
    if instruction == 'F' {
      instruction = ship.direction;
    }

    match instruction {
      'R' => ship.direction = turn(ship.direction, value),
      'L' => ship.direction = turn(ship.direction, -value),
      'N' => ship.north += value,
      'S' => ship.north -= value,
      'E' => ship.east += value,
      'W' => ship.east -= value,
      _ => (),
    }
  }

  Some((i32::abs(ship.north) + i32::abs(ship.east)).to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let instructions = parse(input);
  let mut ship = Ship {
    north: 0,
    east: 0,
    direction: 'E',
  };
  let mut waypoint = Waypoint { east: 10, north: 1 };

  for (instruction, value) in instructions {
    match instruction {
      'N' => waypoint.north += value,
      'S' => waypoint.north -= value,
      'E' => waypoint.east += value,
      'W' => waypoint.east -= value,
      'R' | 'L' => {
        let direction = if instruction == 'R' { 1 } else { -1 };
        for _ in 0..value / 90 {
          let east = waypoint.east;
          waypoint.east = direction * waypoint.north;
          waypoint.north = direction * -east;
        }
      }
      'F' => {
        ship.east += waypoint.east * value;
        ship.north += waypoint.north * value;
      }
      _ => (),
    }
  }

  Some((i32::abs(ship.north) + i32::abs(ship.east)).to_string())
}
