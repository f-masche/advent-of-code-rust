use crate::puzzle::Puzzle;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
  static ref HGT_PATTERN: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
  static ref HCL_PATTERN: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
  static ref ECL_PATTERN: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
  static ref PID_PATTERN: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

pub fn run() {
  let a = Puzzle {
    name: "04-a",
    solution: solution_a,
  };
  a.run_test("2");
  a.run();

  let b = Puzzle {
    name: "04-b",
    solution: solution_b,
  };
  b.run_test("4");
  b.run();
}

fn validate_passport(passport: &HashMap<String, String>) -> bool {
  passport.contains_key("byr")
    && passport.contains_key("iyr")
    && passport.contains_key("eyr")
    && passport.contains_key("hgt")
    && passport.contains_key("hcl")
    && passport.contains_key("ecl")
    && passport.contains_key("pid")
}

fn validate_values(passport: &HashMap<String, String>) -> bool {
  let byr_valid = {
    let byr = passport["byr"].parse::<u32>().unwrap();
    byr >= 1920 && byr <= 2002
  };

  let iyr_valid = {
    let iyr = passport["iyr"].parse::<u32>().unwrap();
    iyr >= 2010 && iyr <= 2020
  };

  let eyr_valid = {
    let eyr = passport["eyr"].parse::<u32>().unwrap();
    eyr >= 2020 && eyr <= 2030
  };

  let hgt_valid: bool = {
    if HGT_PATTERN.is_match(&passport["hgt"]) {
      let hgt_captures = HGT_PATTERN.captures(&passport["hgt"]).unwrap();
      let val = hgt_captures[1].parse::<u32>().unwrap();
      let unit = hgt_captures[2].to_string();
      unit == "cm" && val >= 150 && val <= 193 || unit == "in" && val >= 59 && val <= 76
    } else {
      false
    }
  };

  let hcl_valid = HCL_PATTERN.is_match(&passport["hcl"]);

  let ecl_valid = ECL_PATTERN.is_match(&passport["ecl"]);

  let pid_valid = PID_PATTERN.is_match(&passport["pid"]);

  byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
}

fn parse_passport(input: &String) -> HashMap<String, String> {
  input.split(' ').fold(HashMap::new(), |mut pass, x| {
    let parts: Vec<&str> = x.split(':').collect();
    pass.insert(parts[0].to_string(), parts[1].to_string());
    pass
  })
}

fn parse_passports(batch: &Vec<String>) -> Vec<HashMap<String, String>> {
  let mut passports = Vec::new();
  let mut passport_line: Vec<String> = Vec::new();

  for line in batch.iter() {
    if line.trim().is_empty() {
      passports.push(parse_passport(&(passport_line.join(" "))));
      passport_line = Vec::new();
    } else {
      passport_line.push(line.to_string());
    }
  }

  passports.push(parse_passport(&(passport_line.join(" "))));

  passports
}

fn solution_a(input: &Vec<String>) -> Option<String> {
  let passports = parse_passports(input);
  let count = passports
    .iter()
    .filter(|passport| validate_passport(passport))
    .count();

  Some(count.to_string())
}

fn solution_b(input: &Vec<String>) -> Option<String> {
  let passports = parse_passports(input);
  let count = passports
    .iter()
    .filter(|passport| validate_passport(passport))
    .filter(|passport| validate_values(passport))
    .count();

  Some(count.to_string())
}
