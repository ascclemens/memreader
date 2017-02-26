extern crate memreader;

use std::env::args;
use std::io::Read;

use memreader::{MemReader, ProvidesSlices};

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

  let mut bytes: Vec<u8> = vec![0; n];
  let res = reader.address_slice_len(address, n).read_exact(&mut bytes).unwrap();
  println!("{:?}", res);
}
