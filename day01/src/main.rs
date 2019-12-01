const INPUT: &str = include_str!("../input.txt");

// formula mass -> fuel
fn module_fuel(mass: i32) -> i32 {
  mass / 3 - 2
}

// total fuel required
fn total_fuel() -> i32 {
  INPUT
    .lines()
    .map(|m| module_fuel(m.parse().expect("cannot parse mass")))
    .sum()
}

// compute extra fuel required given some fuel
fn dependent_fuel(fuel: i32) -> i32 {
  let extra = module_fuel(fuel);

  if extra <= 0 {
    0
  } else {
    extra + dependent_fuel(extra)
  }
}

// total fuel required plus the extra dependent fuel
fn recursive_total_fuel() -> i32 {
  INPUT
    .lines()
    .map(|m| module_fuel(m.parse().expect("cannot parse mass")))
    .map(|fuel| fuel + dependent_fuel(fuel))
    .sum()
}

fn main() {
  println!("1st answer: {}", total_fuel());
  println!("2nd answer: {}", recursive_total_fuel());
}
