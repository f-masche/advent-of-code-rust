use crate::puzzle::Puzzle;
use crate::utils::chinese_remainder;

pub fn run() {
  let a = Puzzle {
    name: "13-a",
    solution: solution_a,
  };
  a.run_test("295");
  a.run();

  let b = Puzzle {
    name: "13-b",
    solution: solution_b,
  };
  b.run_test("1068781");
  b.run();
}

fn parse(input: &str) -> (i64, Vec<i64>) {
  let mut lines = input.lines();

  let timestamp = lines.next().unwrap().parse::<i64>().unwrap();
  let bus_ids: Vec<i64> = lines
    .next()
    .unwrap()
    .split(',')
    .map(|bus| if bus == "x" { "0" } else { bus })
    .map(|bus| bus.parse::<i64>().unwrap())
    .collect();
  (timestamp, bus_ids)
}

fn solution_a(input: &String) -> Option<String> {
  let (timestamp, bus_ids) = parse(input);
  let (bus, waiting_time) = bus_ids
    .into_iter()
    .filter(|&bus_id| bus_id > 0)
    .map(|bus_id| {
      (
        bus_id,
        f32::ceil(timestamp as f32 / bus_id as f32) as i64 * bus_id - timestamp,
      )
    })
    .min_by(|(_, x), (_, y)| x.cmp(y))
    .unwrap();

  Some((bus * waiting_time).to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let (_, bus_ids) = parse(input);

  let mut residues = vec![];
  let mut modulii = vec![];

  for (i, &id) in bus_ids.iter().enumerate() {
    if id > 0 {
      modulii.push(id);
      residues.push(id - i as i64);
    }
  }

  let result = chinese_remainder(&residues, &modulii);
  result.map(|x| x.to_string())
}
