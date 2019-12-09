use cgmath::{ulps_eq, Deg, InnerSpace, MetricSpace, Point2, Vector2};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MapCell {
  Empty,
  Asteroid,
}

#[derive(Debug)]
struct SpaceMap {
  grid: Vec<MapCell>,
  width: usize,
  height: usize,
}

fn shitty_deg(deg: f32) -> f32 {
  if deg < 0. {
    360. + deg
  } else {
    deg
  }
}

fn get_map(input: &str) -> SpaceMap {
  let mut grid = Vec::new();
  let mut width = 0;
  let mut height = 0;

  for line in input.lines() {
    if width == 0 {
      width = line.len();
    }

    grid.extend(line.bytes().map(|b| {
      if b == b'.' {
        MapCell::Empty
      } else {
        MapCell::Asteroid
      }
    }));

    height += 1;
  }

  SpaceMap {
    grid,
    width,
    height,
  }
}

fn part_1(map: &SpaceMap) -> Option<(Point2<f32>, Vec<(Vector2<f32>, f32)>)> {
  // map an asteroid ID to everything it sees, a map that associates a vector to the nearest
  // asteroid
  let mut visible = HashMap::new();

  for (i, _) in map
    .grid
    .iter()
    .enumerate()
    .filter(|(_, c)| **c == MapCell::Asteroid)
  {
    let origin = Point2::new((i % map.width) as f32, (i / map.width) as f32);

    // by default, that asteroid sees nothing
    visible.insert(i, Vec::new());

    // for all other asteroids, we’re going to compute the normalized vector between our initial asteroid
    // and all other ones and lookup in teh map; if we find a nearer asteroid, we update
    for (j, _) in map
      .grid
      .iter()
      .enumerate()
      .filter(|(j, cell)| *j != i && **cell == MapCell::Asteroid)
    {
      let p = Point2::new((j % map.width) as f32, (j / map.width) as f32);
      let v = p - origin;
      let nv = v.normalize();
      let magnitude = v.magnitude();

      // first, check if any of the already visible asteroids are on that same line
      let nearests = visible.get_mut(&i).unwrap();

      let mut found = false;
      for (ref mut nearest_nv, ref mut nearest_magnitude) in nearests.iter_mut() {
        if ulps_eq!(*nearest_nv, nv) {
          // we already have found something for that direction; update it if needed

          if magnitude < *nearest_magnitude {
            *nearest_nv = nv;
            *nearest_magnitude = magnitude;
          }

          found = true;
          break;
        }
      }

      if !found {
        nearests.push((nv, magnitude));
      }
    }
  }

  visible
    .iter()
    .max_by_key(|(_, nearests)| nearests.len())
    .map(|(i, nearests)| {
      let p = Point2::new((i % map.width) as f32, (i / map.width) as f32);
      (p, nearests.clone())
    })
}

fn part_2(map: &SpaceMap, center: Point2<f32>) -> Point2<f32> {
  let mut vectors: Vec<(Vector2<f32>, Vec<Point2<f32>>)> = Vec::new();

  // first, we compute all the vectors to all asteroids from our center
  for (i, cell) in map.grid.iter().enumerate() {
    let p = Point2::new((i % map.width) as f32, (i / map.width) as f32);

    if *cell != MapCell::Asteroid || p == center {
      continue;
    }

    let v = p - center;
    let nv = v.normalize();

    let mut found = false;
    for (ref vector, ref mut aligned) in &mut vectors {
      if ulps_eq!(*vector, nv) {
        aligned.push(p);
        found = true;
        break;
      }
    }

    if !found {
      vectors.push((nv, vec![p]));
    }
  }

  for (_, ref mut asteroids) in &mut vectors {
    asteroids.sort_by(|a, b| a.distance(center).partial_cmp(&b.distance(center)).unwrap());
  }

  let up = Vector2::new(0., -1.);
  vectors.sort_by(|(a, _), (b, _)| {
    shitty_deg(Deg::from(up.angle(*a)).0)
      .partial_cmp(&shitty_deg(Deg::from(up.angle(*b)).0))
      .unwrap()
  });

  // c’est DÉGUEULAAAAAAAAAAAASSE
  let mut i = 0;
  let mut nth = 0;
  loop {
    let j = i % vectors.len();
    eprintln!(
      "removing {} {} {} {:?} (angle={:?})",
      nth,
      i,
      j,
      vectors[j].1[0],
      shitty_deg(Deg::from(up.angle(vectors[j].0)).0)
    );

    let v = vectors[j].1.remove(0); // delete the nearest asteroid

    nth += 1;

    if nth == 200 {
      return v;
    }

    if vectors[j].1.is_empty() {
      vectors.remove(j);
    } else {
      i += 1;
    }
  }
}

fn main() {
  let map = get_map(INPUT);

  // first part
  let p1 = part_1(&map).unwrap();
  println!("part 1: {:?} at {:?}", p1.1.len(), p1.0);

  let p2 = part_2(&map, p1.0);
  println!("part 2: {}", p2.x * 100. + p2.y);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    const TEST_INPUT: &str = r#".#..#
.....
#####
....#
...##"#;

    let map = get_map(TEST_INPUT);
    let p = part_1(&map).unwrap();
    eprintln!("{:?}", p.0);

    assert_eq!(p.1.len(), 8);
  }

  #[test]
  fn test_2() {
    const TEST_INPUT: &str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

    let map = get_map(TEST_INPUT);
    let p = part_1(&map).unwrap();
    eprintln!("{:?}", p.0);

    assert_eq!(p.1.len(), 210);
  }

  #[test]
  fn vaporized_1() {
    const TEST_INPUT: &str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

    let map = get_map(TEST_INPUT);
    let p = part_1(&map).unwrap();
    let q = part_2(&map, p.0);
    assert_eq!(q.x * 100. + q.y, 802.);
  }

  #[test]
  fn vaporized_2() {
    const TEST_INPUT: &str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

    let map = get_map(TEST_INPUT);
    let p = part_1(&map).unwrap();
    let q = part_2(&map, p.0);
    assert_eq!(q.x * 100. + q.y, 802.);
  }

  #[test]
  fn test_shitty_deg() {
    assert_eq!(shitty_deg(0.), 0.);
    assert_eq!(shitty_deg(45.), 45.);
    assert_eq!(shitty_deg(90.), 90.);
    assert_eq!(shitty_deg(-45.), 315.);
    assert_eq!(shitty_deg(-1.), 359.);
    assert_eq!(shitty_deg(180.), 180.);

    assert_eq!(
      shitty_deg(Deg::from(Vector2::new(0., -1.).angle(Vector2::new(1., 0.))).0),
      90.
    );

    assert_eq!(
      shitty_deg(Deg::from(Vector2::new(0., -1.).angle(Vector2::new(1., -1.))).0),
      45.
    );
  }
}
