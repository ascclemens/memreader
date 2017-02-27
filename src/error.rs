use std::error::Error;
use std::result::Result as StdResult;
use std::io::Error as IoError;
use std::fmt;

/// Result type for `MemReaderError`.
pub type Result<T> = StdResult<T, MemReaderError>;

/// An error when using `MemReader`.
#[derive(Debug)]
pub enum MemReaderError {
  /// Fewer bytes were read from memory than were requested.
  ///
  /// Contains the number of bytes read and the shortened vector.
  FewerBytesRead(usize, Vec<u8>),

  /// The attempt to read the memory was unsuccessful.
  ///
  /// If there was any error status from the system call, it is included.
  UnsuccessfulRead(Option<isize>),

  /// Unable to get a handle on the process to read.
  ///
  /// If there was any error status from the system call, it is included.
  Handle(Option<isize>),

  /// An IO error occurred and is included.
  Io(IoError),

  /// Another error, represented by a code.
  Other(isize)
}

impl fmt::Display for MemReaderError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

impl Error for MemReaderError {
    fn description(&self) -> &str {
      ""
    }

    fn cause(&self) -> Option<&Error> {
      None
    }
}
