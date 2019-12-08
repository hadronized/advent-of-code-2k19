const INPUT: &str = include_str!("../input.txt");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn read_image(input: &str, width: usize, height: usize) -> Vec<Vec<u8>> {
  input
    .trim()
    .as_bytes()
    .chunks(width * height)
    .map(|chunk| chunk.iter().cloned().collect())
    .collect()
}

fn part1(image: &Vec<Vec<u8>>) -> usize {
  let (layer, _) =
    image
      .iter()
      .enumerate()
      .fold((0, usize::max_value()), |(layer, zeros), (i, pixels)| {
        let count = pixels.iter().filter(|&&p| p == b'0').count();
        if count < zeros {
          (i, count)
        } else {
          (layer, zeros)
        }
      });

  let (ones, twos) = image[layer].iter().fold((0, 0), |(ones, twos), &p| {
    if p == b'1' {
      (ones + 1, twos)
    } else if p == b'2' {
      (ones, twos + 1)
    } else {
      (ones, twos)
    }
  });

  ones * twos
}

fn part2(image: &Vec<Vec<u8>>, width: usize, height: usize) -> Vec<char> {
  let mut framebuffer = vec![' '; width * height];

  for layer in image.iter().rev() {
    for y in 0..height {
      for x in 0..width {
        let index = x + y * width;
        let dst = layer[index];

        if dst == b'0' {
          framebuffer[index] = ' ';
        } else if dst == b'1' {
          framebuffer[index] = '█';
        }
      }
    }
  }

  framebuffer
}

fn main() {
  let image = read_image(INPUT, WIDTH, HEIGHT);

  println!("1st answer: {}", part1(&image));

  let blended = part2(&image, WIDTH, HEIGHT);

  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let p = blended[x + y * WIDTH];
      print!("{}", p);
    }

    println!();
  }
}
