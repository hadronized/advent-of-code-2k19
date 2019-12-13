use intcode::{Program, Word};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> HashMap<[Word; 2], Word> {
  let mut program = Program::from_str(INPUT.trim()).unwrap();
  let mut tiles = HashMap::new();

  loop {
    let x = if let Some(output) = program.run_suspended(&[]).unwrap().output() {
      output
    } else {
      break tiles;
    };

    let y = program.run_suspended(&[]).unwrap().output().unwrap();
    let tile = program.run_suspended(&[]).unwrap().output().unwrap();
    tiles.insert([x, y], tile);
  }
}

fn part_2(input: &str) -> Word {
  let mut program = Program::from_str(input.trim()).unwrap();
  program.write(0, 2).unwrap();

  let mut paddle = None;
  let mut ball = None;
  let mut next_input = 0;
  let mut score = 0;

  loop {
    let x = if let Some(output) = program.run_suspended(&[next_input]).unwrap().output() {
      output
    } else {
      break score;
    };

    next_input = 0;

    let y = program.run_suspended(&[]).unwrap().output().unwrap();
    let tile = program.run_suspended(&[]).unwrap().output().unwrap();

    if x == -1 && y == 0 {
      score = tile;
    }

    // update paddle
    match tile {
      3 => paddle = Some([x, y]),
      4 => ball = Some([x, y]),
      _ => (),
    }

    if let (Some(paddle), Some(ball)) = (paddle, ball) {
      if paddle[0] < ball[0] {
        next_input = 1;
      } else if paddle[0] > ball[0] {
        next_input = -1;
      }
    }
  }
}

fn main() {
  let p1 = part_1(INPUT)
    .into_iter()
    .filter(|(_, tile)| *tile == 2)
    .count();
  println!("Part 1: {}", p1);

  let score = part_2(INPUT);
  println!("Part 2: {}", score);
}
