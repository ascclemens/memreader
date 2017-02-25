extern crate memreader;

use std::env::args;
use memreader::{MemReader, ReadsMemory};

fn main() {
  let args: Vec<String> = args().skip(1).collect();

  if args.len() < 3 {
    println!("read [pid] [address] [n]");
    return;
  }

  let pid: u32 = args[0].parse().unwrap();
  let address: usize = args[1].parse().unwrap();
  let n: usize = args[2].parse().unwrap();

  let reader = MemReader::new(pid).unwrap();

  println!("{:?}", reader.read_bytes(address, n));
}
