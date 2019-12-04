const INPUT_LOWER: Bucket = [1, 3, 4, 5, 6, 4];
const INPUT_UPPER: Bucket = [5, 8, 5, 1, 5, 9];

type Bucket = [u8; 6];

#[cfg(feature = "part-1")]
fn adjacency_rule(bucket: &Bucket) -> bool {
  for i in 0..5 {
    if bucket[i] == bucket[i + 1] {
      return true;
    }
  }

  return false;
}

#[cfg(feature = "part-2")]
fn adjacency_rule(bucket: &Bucket) -> bool {
  let mut c = 0;

  for i in 0..5 {
    if bucket[i] == bucket[i + 1] {
      c += 1;
    } else if c == 1 {
      return true;
    } else {
      c = 0;
    }
  }

  return c == 1;
}

fn count(mut bucket: Bucket, max: Bucket) -> usize {
  let mut i = 5;
  let mut result = 0;

  loop {
    if bucket >= max {
      break result;
    }

    // we are at the end of the bucket and the adjacency rule is satisfied
    if i == 5 {
      if adjacency_rule(&bucket) {
        result += 1;
      }

      // backtrack
      while i > 0 && bucket[i] == 9 {
        i -= 1;
      }

      // rebuild the bucket from the current value
      let x = bucket[i] + 1;
      while i < 6 {
        bucket[i] = x;
        i += 1;
      }

      i = 5;
    }
  }
}

fn main() {
  let result = count(INPUT_LOWER, INPUT_UPPER);
  println!("answer: {}", result);
}
