use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

/// A wire is made out of cells on the 2D signed integer plane.
#[derive(Debug)]
struct Wire {
  cells: Vec<[i32; 2]>,
}

fn get_wires(input: &str) -> Result<Vec<Wire>, String> {
  input
    .lines()
    .map(|l| &l[0..l.len() - 1])
    .map(to_wire)
    .collect()
}

fn to_wire(line: &str) -> Result<Wire, String> {
  let mut cells = vec![[0, 0]];

  for dir in line.split(',') {
    if dir.is_empty() {
      return Err("malformed direction".to_owned());
    }

    let value = dir[1..]
      .parse()
      .map_err(|e| format!("cannot parse direction value: {} {}", e, dir))?;

    match dir.as_bytes()[0] {
      b'L' => {
        let [px, py] = cells.last().cloned().unwrap();
        let new_cells = (0..value).map(|v| [px - v - 1, py]);
        cells.extend(new_cells);
      }

      b'R' => {
        let [px, py] = cells.last().cloned().unwrap();
        let new_cells = (0..value).map(|v| [px + v + 1, py]);
        cells.extend(new_cells);
      }

      b'U' => {
        let [px, py] = cells.last().cloned().unwrap();
        let new_cells = (0..value).map(|v| [px, py + v + 1]);
        cells.extend(new_cells);
      }

      b'D' => {
        let [px, py] = cells.last().cloned().unwrap();
        let new_cells = (0..value).map(|v| [px, py - v - 1]);
        cells.extend(new_cells);
      }

      x => return Err(format!("wrong direction: {}", x)),
    }
  }

  Ok(Wire { cells })
}

fn closest_intersection(wires: &[Wire]) -> Option<[i32; 2]> {
  let mut overlaps = HashMap::<[i32; 2], [bool; 2]>::new();

  for (i, wire) in wires.iter().enumerate() {
    let mut presence = [false, false];
    presence[i] = true;

    for &cell in &wire.cells {
      overlaps.entry(cell).or_insert(presence)[i] = true;
    }
  }

  let mut intersections: Vec<_> = overlaps
    .into_iter()
    .filter(|(p, wire_ids)| *p != [0, 0] && wire_ids[0] && wire_ids[1])
    .collect();
  intersections.sort_by(|(ap, _), (bp, _)| dist(*ap, [0, 0]).cmp(&dist(*bp, [0, 0])));

  intersections.get(0).map(|(d, _)| *d)
}

fn dist(a: [i32; 2], b: [i32; 2]) -> u32 {
  let x = (b[0] - a[0]).abs();
  let y = (b[1] - a[1]).abs();
  (x + y) as u32
}

fn intersections(wires: &[Wire]) -> Option<Vec<[i32; 2]>> {
  let mut overlaps = HashMap::<[i32; 2], [bool; 2]>::new();

  for (i, wire) in wires.iter().enumerate() {
    let mut presence = [false, false];
    presence[i] = true;

    for &cell in &wire.cells {
      overlaps.entry(cell).or_insert(presence)[i] = true;
    }
  }

  let intersections: Vec<_> = overlaps
    .into_iter()
    .filter(|(p, wire_ids)| *p != [0, 0] && wire_ids[0] && wire_ids[1])
    .map(|(p, _)| p)
    .collect();

  Some(intersections)
}

fn steps(wire: &Wire, p: [i32; 2]) -> usize {
  let mut steps = 0;

  for &q in &wire.cells {
    if q == p {
      break;
    }

    steps += 1;
  }

  steps
}

fn best_steps(wires: &[Wire]) -> Option<usize> {
  let xs = intersections(wires)?;

  let mut best = usize::max_value();

  for x in xs {
    best = best.min(wires.iter().map(|wire| steps(wire, x)).sum());
  }

  Some(best)
}

fn main() {
  let wires = get_wires(INPUT).unwrap();
  let intersection = closest_intersection(&wires);

  println!("1st answer: {:?}", intersection);
  println!("2nd answer: {:?}", best_steps(&wires));
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT_TEST_1: &str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
  const INPUT_TEST_2: &str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R07"#;

  #[test]
  fn test_distance_1() {
    let wires = get_wires(INPUT_TEST_1).unwrap();
    let result = dist(closest_intersection(&wires).unwrap(), [0, 0]);
    assert_eq!(result, 159);
  }

  #[test]
  fn test_distance_2() {
    let wires = get_wires(INPUT_TEST_2).unwrap();
    let result = dist(closest_intersection(&wires).unwrap(), [0, 0]);
    assert_eq!(result, 135);
  }

  #[test]
  fn test_steps_1() {
    let wires = get_wires(INPUT_TEST_1).unwrap();
    let result = best_steps(&wires).unwrap();
    assert_eq!(result, 610);
  }

  #[test]
  fn test_steps_2() {
    let wires = get_wires(INPUT_TEST_2).unwrap();
    let result = best_steps(&wires).unwrap();
    assert_eq!(result, 410);
  }
}
