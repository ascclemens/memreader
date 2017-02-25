use libc::c_int;

use ReadsMemory;

struct MemReader;

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>, c_int> {
    unimplemented!();
  }
}
