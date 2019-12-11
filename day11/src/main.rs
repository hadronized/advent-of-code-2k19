use intcode::{Program, Word};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn part_1(program: &mut Program, starting_panel: Word) -> HashMap<[Word; 2], Word> {
  let mut grid = HashMap::new();
  let mut dir = [0, 1]; // up
  let mut robot_pos = [0, 0];

  loop {
    let suspended = program
      .run_suspended(&[grid.get(&robot_pos).cloned().unwrap_or(starting_panel)])
      .unwrap();

    if suspended.output().is_none() {
      break;
    }

    *grid.entry(robot_pos).or_insert(0) = suspended.output().unwrap();

    let suspended = program.run_suspended(&[]).unwrap();
    let next_dir = suspended.output().unwrap();
    match dir {
      [0, 1] => {
        dir = [-1 + 2 * next_dir, 0];
        robot_pos = [robot_pos[0] - 1 + 2 * next_dir, robot_pos[1]];
      }

      [-1, 0] => {
        dir = [0, -1 + 2 * next_dir];
        robot_pos = [robot_pos[0], robot_pos[1] - 1 + 2 * next_dir];
      }

      [0, -1] => {
        dir = [1 - 2 * next_dir, 0];
        robot_pos = [robot_pos[0] + 1 - 2 * next_dir, robot_pos[1]];
      }

      [1, 0] => {
        dir = [0, 1 - 2 * next_dir];
        robot_pos = [robot_pos[0], robot_pos[1] + 1 - 2 * next_dir];
      }

      _ => panic!("wrong dir: {:?}", dir),
    }
  }

  grid
}

fn main() {
  let mut program = Program::from_str(INPUT.trim()).unwrap();
  let p1 = part_1(&mut program, 0);

  println!("Part 1: {}", p1.len());

  let mut program = Program::from_str(INPUT.trim()).unwrap();
  let p2 = part_1(&mut program, 1);

  let mut map = vec![' '; 60 * 60];
  for ([x, y], c) in p2 {
    if c == 1 {
      map[((y + 30) * 60 + (x + 40)) as usize] = '█';
    }
  }

  for y in 0..60 {
    for x in 0..60 {
      print!("{}", map[x + (59 - y) * 60]);
    }

    println!();
  }
}
