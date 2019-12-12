use num::Integer;

const INPUT: &str = include_str!("../input.txt");
const INITIAL_VELOCITY: [i64; 3] = [0, 0, 0];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Moon {
  pos: [i64; 3],
  vel: [i64; 3],
}

impl Moon {
  fn pot(&self) -> u64 {
    self.pos.iter().copied().map(|x| x.abs() as u64).sum()
  }

  fn kin(&self) -> u64 {
    self.vel.iter().copied().map(|x| x.abs() as u64).sum()
  }

  fn tot(&self) -> u64 {
    self.pot() * self.kin()
  }
}

fn get_moons(input: &str) -> Vec<Moon> {
  input
    .lines()
    .map(|line| {
      let xyz = line.trim()[1..line.len() - 1]
        .split(',')
        .map(|coord| coord.trim()[2..].parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

      Moon {
        pos: [xyz[0], xyz[1], xyz[2]],
        vel: INITIAL_VELOCITY,
      }
    })
    .collect()
}

fn update_vel(moons: &mut [Moon]) {
  for a in 0..moons.len() - 1 {
    for b in a + 1..moons.len() {
      // gravity
      for axis in 0..3 {
        let vel = (moons[b].pos[axis] - moons[a].pos[axis]).signum();
        moons[a].vel[axis] += vel;
        moons[b].vel[axis] -= vel;
      }
    }
  }
}

fn update_pos(moons: &mut [Moon]) {
  for moon in moons {
    for axis in 0..3 {
      moon.pos[axis] += moon.vel[axis];
    }
  }
}

fn system_energy(moons: &[Moon]) -> u64 {
  moons.iter().map(Moon::tot).sum()
}

fn simulate(moons: &mut [Moon], iterations: usize) -> u64 {
  for _ in 0..iterations {
    update_vel(moons);
    update_pos(moons);
  }

  system_energy(moons)
}

fn find_cycle(moons: &mut [Moon]) -> usize {
  let seen_x = [
    moons[0].pos[0],
    moons[1].pos[0],
    moons[2].pos[0],
    moons[3].pos[0],
    0,
    0,
    0,
    0,
  ];
  let seen_y = [
    moons[0].pos[1],
    moons[1].pos[1],
    moons[2].pos[1],
    moons[3].pos[1],
    0,
    0,
    0,
    0,
  ];
  let seen_z = [
    moons[0].pos[2],
    moons[2].pos[2],
    moons[2].pos[2],
    moons[3].pos[2],
    0,
    0,
    0,
    0,
  ];
  let mut found_x = None;
  let mut found_y = None;
  let mut found_z = None;
  let mut step = 0;

  loop {
    update_vel(moons);
    update_pos(moons);

    step += 1;

    let key_x = [
      moons[0].pos[0],
      moons[1].pos[0],
      moons[2].pos[0],
      moons[3].pos[0],
      moons[0].vel[0],
      moons[1].vel[0],
      moons[2].vel[0],
      moons[3].vel[0],
    ];
    let key_y = [
      moons[0].pos[1],
      moons[1].pos[1],
      moons[2].pos[1],
      moons[3].pos[1],
      moons[0].vel[1],
      moons[1].vel[1],
      moons[2].vel[1],
      moons[3].vel[1],
    ];
    let key_z = [
      moons[0].pos[2],
      moons[2].pos[2],
      moons[2].pos[2],
      moons[3].pos[2],
      moons[0].vel[2],
      moons[1].vel[2],
      moons[2].vel[2],
      moons[3].vel[2],
    ];

    if found_x.is_none() && key_x == seen_x {
      found_x = Some(step);
    }

    if found_y.is_none() && key_y == seen_y {
      found_y = Some(step);
    }

    if found_z.is_none() && key_z == seen_z {
      found_z = Some(step);
    }

    if found_x.is_some() && found_y.is_some() && found_z.is_some() {
      break;
    }
  }

  let lcm_xy = found_x.unwrap().lcm(&found_y.unwrap());
  let lcm_z = lcm_xy.lcm(&found_z.unwrap());

  lcm_z
}

fn main() {
  let mut moons = get_moons(INPUT);
  let energy = simulate(&mut moons, 1000);
  println!("part 1: {}", energy);

  let mut moons = get_moons(INPUT);
  let cycle = find_cycle(&mut moons);
  println!("part 2: {}", cycle);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn total_energy() {
    const INPUT_TEST: &str = r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#;
    let mut moons = get_moons(INPUT_TEST);

    let energy = simulate(&mut moons, 10);
    assert_eq!(energy, 179);
  }
}
