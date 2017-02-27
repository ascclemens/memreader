use std::io::Read;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use {MemReader, ReadsMemory};

pub struct MemorySlice<'a> {
  start: usize,
  end: usize,
  reader: &'a MemReader,
  mark: Option<usize>
}

impl<'a> MemorySlice<'a> {
  pub fn new(reader: &'a MemReader, start: usize, end: usize) -> Self {
    MemorySlice {
      start: start,
      end: end,
      reader: reader,
      mark: None
    }
  }

  /// The start address of this slice.
  pub fn start(&self) -> usize {
    self.start
  }

  /// The end address of this slice.
  pub fn end(&self) -> usize {
    self.end
  }
}

impl<'a> Read for MemorySlice<'a> {
  fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
    let start = match self.mark {
      Some(m) => m,
      None => self.start
    };
    let buf_len = buf.len();
    if start + buf_len > self.end {
      return Ok(0);
    }
    let bytes = self.reader.read_bytes(start, buf_len).map_err(|e| IoError::new(IoErrorKind::Other, e))?;
    let bytes_len = bytes.len();
    self.mark = Some(start + buf_len);
    for (i, byte) in bytes.into_iter().enumerate() {
      buf[i] = byte;
    }
    Ok(bytes_len)
  }
}
