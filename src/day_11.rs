use crate::puzzle::Puzzle;

pub fn run() {
  let a = Puzzle {
    name: "11-a",
    solution: solution_a,
  };
  a.run_test("37");
  a.run();

  let b = Puzzle {
    name: "11-b",
    solution: solution_b,
  };
  b.run_test("26");
  b.run();
}

fn parse_seats(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|line| line.chars().collect()).collect()
}

fn is_free(seat: char) -> bool {
  seat == 'L'
}

fn is_occupied(seat: char) -> bool {
  seat == '#'
}

fn occupied_count(x: usize, y: usize, seats: &Vec<Vec<char>>) -> usize {
  let mut count = 0;

  for y_offset in 0..3 {
    for x_offset in 0..3 {
      // skip center
      if y_offset == 1 && x_offset == 1 {
        continue;
      }

      // top & left boundaries
      if x + x_offset == 0 || y + y_offset == 0 {
        continue;
      }

      let seat_y = y + y_offset - 1;
      let seat_x = x + x_offset - 1;

      // bottom & right boundaries
      if seat_y > seats.len() - 1 || seat_x > seats[0].len() - 1 {
        continue;
      }

      if is_occupied(seats[seat_y][seat_x]) {
        count += 1;
      }
    }
  }
  count
}

fn occupied_count_b(x: usize, y: usize, seats: &Vec<Vec<char>>) -> usize {
  let mut count = 0;
  let width = seats[0].len();
  let height = seats.len();

  let mut eval = |y: usize, x: usize| -> bool {
    let seat = seats[y][x];
    if is_occupied(seat) {
      count += 1;
      return true;
    }
    is_free(seat)
  };

  // up
  for y_offset in (0..y).rev() {
    if eval(y_offset, x) {
      break;
    }
  }

  // down
  for y_offset in y + 1..height {
    if eval(y_offset, x) {
      break;
    }
  }

  // left
  for x_offset in (0..x).rev() {
    if eval(y, x_offset) {
      break;
    }
  }

  // right
  for x_offset in x + 1..width {
    if eval(y, x_offset) {
      break;
    }
  }

  // up left
  for (y_offset, x_offset) in (0..y).rev().zip((0..x).rev()) {
    if eval(y_offset, x_offset) {
      break;
    }
  }

  // down left
  for (y_offset, x_offset) in (y + 1..height).zip((0..x).rev()) {
    if eval(y_offset, x_offset) {
      break;
    }
  }

  // up right
  for (y_offset, x_offset) in (0..y).rev().zip(x + 1..width) {
    if eval(y_offset, x_offset) {
      break;
    }
  }

  // down right
  for (y_offset, x_offset) in (y + 1..height).zip(x + 1..width) {
    if eval(y_offset, x_offset) {
      break;
    }
  }

  count
}

fn simulate(
  seats: &Vec<Vec<char>>,
  max_occupied: usize,
  get_occupied: fn(usize, usize, &Vec<Vec<char>>) -> usize,
) -> u32 {
  let mut current = seats.clone();
  let mut next: Vec<Vec<char>>;
  let mut change = true;
  let mut occupied = 0;

  while change {
    change = false;
    next = current.clone();

    for y in 0..current.len() {
      for x in 0..current[y].len() {
        let seat = current[y][x];
        let count = get_occupied(x, y, &current);

        if is_free(seat) && count == 0 {
          change = true;
          occupied += 1;
          next[y][x] = '#';
        } else if is_occupied(seat) && count >= max_occupied {
          change = true;
          occupied -= 1;
          next[y][x] = 'L';
        }
      }
    }

    current = next;
  }

  occupied
}

fn solution_a(input: &String) -> Option<String> {
  let seats = parse_seats(input);
  let occupied = simulate(&seats, 4, occupied_count);
  Some(occupied.to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let seats = parse_seats(input);
  let occupied = simulate(&seats, 5, occupied_count_b);
  Some(occupied.to_string())
}
