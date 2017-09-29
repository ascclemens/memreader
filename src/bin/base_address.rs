extern crate memreader;

use std::env::args;

use memreader::prelude::*;

fn main() {
  let args: Vec<String> = args().skip(1).collect();

  if args.len() < 2 {
    println!("base_address [pid] [name]");
    return;
  }

  let pid: u32 = args[0].parse().unwrap();
  let name = &args[1];

  let reader = MemReader::new(pid).unwrap();

  println!("{:?}", reader.base_address(name));
}
