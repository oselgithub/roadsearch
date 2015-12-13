use std::error::Error;
use std::fmt::{Display, Result, Formatter};
use std::num::ParseIntError;
  
#[derive(Debug,Clone,PartialEq)]
pub enum LibError {
  EntryMissing,
  BadFileFormat,
  ParseIntError(ParseIntError),
}

impl Error for LibError {
  fn description(&self) -> &str {
    match *self {
      LibError::EntryMissing => "entry is missing",
      LibError::BadFileFormat => "a bad file format encountered",
      LibError::ParseIntError(_) => "integer parsing error occurred",
    }
  }
  
  fn cause(&self) -> Option< &Error > {
    match *self {
      LibError::ParseIntError(ref err) => Some(err as &Error),
      _ => None,
    }
  }
}


impl Display for LibError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.description())
  }
}

impl From< ParseIntError > for LibError {
  fn from(err: ParseIntError) -> LibError {
    LibError::ParseIntError(err)
  }
}
