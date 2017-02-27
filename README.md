# memreader

`memreader` is a library written in [Rust](http://rust-lang.org/), designed to read memory from
other processes.

```rust
extern crate memreader;

use memreader::{MemReader, ProvidesSlices};

use std::env::args;

fn main() {
  let args: Vec<String> = args().skip(1).collect();
  if args.len() < 3 {
    return;
  }
  let pid: u32 = args[0].parse().unwrap();
  let addr: usize = args[1].parse().unwrap();
  let n: usize = args[2].parse().unwrap();

  let reader = MemReader::new(pid).unwrap();

  let mut buf = vec![0; n];

  reader.address_slice_len(addr, n).read_exact(&mut buf).unwrap();

  println!("{} bytes at location {} in process {}'s memory: {:?}", n, addr, pid, buf);
}
```
