use crate::puzzle::Puzzle;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Rule {
  color: String,
  content: Vec<Bag>,
}

#[derive(Debug)]
struct Bag {
  color: String,
  count: u8,
}

pub fn run() {
  let a = Puzzle {
    name: "07-a",
    solution: solution_a,
  };
  a.run_test("4");
  a.run();

  let b = Puzzle {
    name: "07-b",
    solution: solution_b,
  };
  b.run_test("32");
  b.run();
}

fn parse_rules(input: &String) -> HashMap<String, Rule> {
  let mut rules: HashMap<String, Rule> = HashMap::new();

  for line in input.lines() {
    let parts: Vec<&str> = line.split("contain").collect();
    let color = &parts[0].split(' ').collect::<Vec<&str>>()[..2].join(" ");

    let content = parts[1].split(',').fold(Vec::new(), |mut content, part| {
      let parts: Vec<&str> = part.trim().split(" ").collect();
      if parts[0] != "no" {
        content.push(Bag {
          color: parts[1..3].join(" ").to_string(),
          count: parts[0].parse::<u8>().unwrap(),
        });
      }
      content
    });

    let rule = Rule {
      color: color.to_string(),
      content: content,
    };

    rules.insert(rule.color.to_string(), rule);
  }

  rules
}

fn can_contain_bag(color: &str, rules: &HashMap<String, Rule>) -> HashSet<String> {
  rules.values().fold(HashSet::new(), |mut all, rule| {
    let bag: Option<&Bag> = rule.content.iter().find(|bag| bag.color == color);

    if bag.is_some() {
      all.insert(rule.color.to_string());
      for color in can_contain_bag(&rule.color, rules) {
        all.insert(color.to_string());
      }
    }
    all
  })
}

fn count_bags(rule: &Rule, rules: &HashMap<String, Rule>) -> u32 {
  rule.content.iter().fold(0u32, |mut sum, bag| {
    let next = &rules[&bag.color];
    sum += (bag.count as u32) * (count_bags(next, rules) + 1);
    sum
  })
}

fn solution_a(input: &String) -> Option<String> {
  let rules = parse_rules(input);
  let result = can_contain_bag("shiny gold", &rules);
  Some(result.len().to_string())
}

fn solution_b(input: &String) -> Option<String> {
  let rules = parse_rules(input);
  let result = count_bags(&rules["shiny gold"], &rules);
  Some(result.to_string())
}
