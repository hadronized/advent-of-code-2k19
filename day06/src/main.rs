use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn pair_from_iter<'a, I>(mut i: I) -> Option<(String, String)>
where
  I: Iterator<Item = &'a str>,
{
  let first = i.next()?;
  let second = i.next()?;
  Some((first.trim().to_owned(), second.trim().to_owned()))
}

fn get_orbit_pairs(input: &str) -> Vec<(String, String)> {
  input
    .lines()
    .filter(|l| !l.trim().is_empty())
    .map(|line| pair_from_iter(line.trim_matches(|c| c == ' ' && c == '\n').split(")")).unwrap())
    .collect()
}

type Orbits = HashMap<String, String>;

fn build_graph(pairs: &[(String, String)]) -> Orbits {
  let mut graph: Orbits = pairs.into_iter().cloned().map(|(a, b)| (b, a)).collect();
  graph.insert("COM".to_owned(), String::new());

  graph
}

fn count_orbits(orbits: &Orbits) -> usize {
  let mut sum = 0;

  for orbit in orbits {
    let mut orbited = orbit.1.clone();
    let mut depth = if orbit.0 != "COM" { 1 } else { 0 };

    loop {
      if orbited == "COM" {
        break;
      }

      if let Some(new_orbit) = orbits.get(&orbited) {
        orbited = new_orbit.clone();
        depth += 1;
      } else {
        break;
      }
    }

    sum += depth;
  }

  sum
}

fn path_to(orbits: &Orbits, mut object: String, dest: String) -> Vec<String> {
  let mut path = Vec::new();

  loop {
    path.push(object.clone());

    if object == dest {
      break;
    } else if let Some(new_orbit) = orbits.get(&object).cloned() {
      object = new_orbit;
    } else {
      break;
    }
  }

  path
}

fn find_1st_intersection(a: &[String], b: &[String]) -> String {
  let mut rev_a = a.iter().rev();
  let mut rev_b = b.iter().rev();
  let mut intersection = a[0].clone();

  loop {
    let a_ = rev_a.next().unwrap();
    let b_ = rev_b.next().unwrap();

    if a_ != b_ {
      break;
    } else {
      intersection = a_.clone();
    }
  }

  intersection
}

fn reduce_path<'a>(mut path: &'a [String], dest: &str) -> &'a [String] {
  loop {
    if path.last().unwrap() == dest {
      break;
    } else {
      path = &path[0..path.len() - 1];
    }
  }

  path
}

fn main() {
  let orbits = build_graph(&get_orbit_pairs(INPUT));
  let count = count_orbits(&orbits);

  println!("1st answer: {}", count);

  let you_com_path = path_to(&orbits, "YOU".to_owned(), "COM".to_owned());
  let santa_com_path = path_to(&orbits, "SAN".to_owned(), "COM".to_owned());

  println!("you -> com: {:?}", you_com_path);
  println!("santa -> com: {:?}", santa_com_path);

  let intersection = find_1st_intersection(&you_com_path, &santa_com_path);
  println!("intersection: {}", intersection);

  let new_you_path = reduce_path(&you_com_path, &intersection);
  let new_santa_path = reduce_path(&santa_com_path, &intersection);

  println!("new path for me: {:?} {}", new_you_path, new_you_path.len());
  println!(
    "new path for santa: {:?} {}",
    new_santa_path,
    new_santa_path.len()
  );

  println!(
    "2nd answer: {}",
    new_you_path.len() + new_santa_path.len() - 4
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_MAP: &str = r#"
    COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
  "#;

  #[test]
  fn test_map() {
    let graph = build_graph(&get_orbit_pairs(TEST_MAP));
    let count = count_orbits(&graph);
    assert_eq!(count, 42);
  }
}
