// John works at a clothing store. He has a large pile of socks that he must pair by color for sale. Given an array of integers representing the color of each sock, determine how many pairs of socks with matching colors there are.

// For example, there are  socks with colors . There is one pair of color  and one of color . There are three odd socks left, one of each color. The number of pairs is .

// Sample Input

// 9
// 10 20 20 10 10 30 50 10 20

use std::io;

fn main() {
  let mut n = String::new();
  let mut arr = String::new();

  io::stdin().read_line(&mut n)
    .expect("cannot read n from line");

  io::stdin().read_line(&mut arr)
    .expect("cannot read arr from line");

  let n: usize = n.trim().parse()
    .expect("expect a number");
  let mut arr: Vec<u8> = arr
    .split_whitespace()
    .map(|s| s.parse().expect("expect a number"))
    .collect();

  let mut result = 0;
  for i in 0..n {
    for j in i + 1..n {
      if arr[i] != 0 && arr[j] != 0 && arr[i] == arr[j] {
        arr[i] = 0;
        arr[j] = 0;
        result = result + 1;
      }
    }
  }
  println!("{}", result);
}