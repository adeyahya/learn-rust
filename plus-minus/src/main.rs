use std::io;

fn main() {
  let mut n = String::new();
  let mut arr = String::new();
  let mut positive: i32 = 0;
  let mut negative: i32 = 0;
  let mut zero: i32 = 0;

  io::stdin().read_line(&mut n).expect("err");
  io::stdin().read_line(&mut arr).expect("err");

  let n: i32 = n.trim().parse().expect("err");
  let arr: Vec<i32> = arr
    .trim()
    .split_whitespace()
    .map(|s| s.trim().parse().expect("err"))
    .collect();

  for item in 0..(n as usize) {
    if arr[item] == 0 {
      zero = zero + 1;
    }

    if arr[item] > 0 {
      positive = positive + 1;
    }

    if arr[item] < 0 {
      negative = negative + 1;
    }
  }

  let positive = positive as f32 / n as f32;
  let negative = negative as f32 / n as f32;
  let zero = zero as f32 / n as f32;
  println!("{}\n{}\n{}", positive, negative, zero);
}
