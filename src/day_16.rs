use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub fn run() {
  let a = Puzzle {
    name: "16-a",
    solution: solution_a,
  };
  a.run_test("71");
  a.run();

  let b = Puzzle {
    name: "16-b",
    solution: solution_b,
  };
  b.run();
}

type Rule = (String, Vec<(u64, u64)>);
type Ticket = Vec<u64>;

fn groups(input: &str) -> Vec<Vec<&str>> {
  input
    .split("\n\n")
    .map(|group| group.lines().collect())
    .collect()
}

fn parse_rules(input: &Vec<&str>) -> Vec<Rule> {
  input
    .iter()
    .map(|line| {
      let parts: Vec<&str> = line.split(": ").collect();
      let field = parts[0];
      let ranges = parts[1]
        .split(" or ")
        .map(|x| {
          let range: Vec<u64> = x.split("-").map(|num| num.parse().unwrap()).collect();
          (range[0], range[1])
        })
        .collect();
      (field.to_string(), ranges)
    })
    .collect()
}

fn parse_ticket(input: &Vec<&str>) -> Ticket {
  input[1].split(',').map(|x| x.parse().unwrap()).collect()
}

fn parse_nearby_tickets(input: &Vec<&str>) -> Vec<Ticket> {
  input
    .iter()
    .skip(1)
    .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
    .collect()
}

fn validate(rule: &Rule, number: u64) -> bool {
  rule.1.iter().fold(false, |valid, (min, max)| {
    valid || number >= *min && number <= *max
  })
}

fn invalid(ticket: &Ticket, rules: &Vec<Rule>) -> Option<u64> {
  for number in ticket.iter() {
    if !rules.iter().any(|rule| validate(&rule, *number)) {
      return Some(*number);
    }
  }
  None
}

fn solution_a(input: &String) -> Option<String> {
  let blocks = groups(input);
  let rules = parse_rules(&blocks[0]);
  let nearby_tickets = parse_nearby_tickets(&blocks[2]);
  let result = nearby_tickets
    .iter()
    .map(|ticket| invalid(&ticket, &rules))
    .filter_map(|x| x)
    .sum::<u64>();

  Some(result.to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let blocks = groups(input);
  let mut rules = parse_rules(&blocks[0]);
  let my_ticket = parse_ticket(&blocks[1]);
  let nearby_tickets: Vec<Ticket> = parse_nearby_tickets(&blocks[2])
    .into_iter()
    .filter(|ticket| invalid(&ticket, &rules).is_none())
    .collect();
  let mut positions = HashMap::new();

  while rules.len() > 0 {
    let rule = &rules[0];
    let valid_pos: Vec<usize> = (0..my_ticket.len())
      .filter(|pos| {
        if positions.values().any(|&x| x == *pos) {
          return false;
        }
        nearby_tickets
          .iter()
          .all(|ticket| validate(rule, ticket[*pos]))
      })
      .collect();

    if valid_pos.len() == 1 {
      positions.insert(rule.0.to_string(), valid_pos[0]);
      rules.remove(0);
    }

    if rules.len() > 0 {
      let tmp = rules.remove(0);
      rules.push(tmp);
    }
  }

  let result: u64 = positions
    .keys()
    .filter(|key| key.starts_with("departure"))
    .map(|key| my_ticket[positions[key]])
    .product();

  Some(result.to_string())
}
