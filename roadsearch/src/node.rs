
use std::str::FromStr;

use error::LibError;

#[derive(Debug,Clone,PartialEq)]
pub struct Node {
  name: String,
  id: u32,
  x: i32,
  y: i32,
}

impl Node {
  pub fn new(name: &str, id: u32, x: i32, y: i32) -> Node {
    Node {
      name: name.to_string(),
      id: id,
      x: x,
      y: y,
    }
  }

  pub fn distance(&self, node: &Node) -> f32 {
    if self.x == node.x {
      return (self.y - node.y).abs() as f32;
    }
    if self.y == node.y {
      return (self.x - node.x).abs() as f32;
    }
    return (((self.x - node.x).pow(2) + (self.y - node.y).pow(2)) as f32).sqrt();
  }
  
  fn parse_number(item: Option< &str >) -> Result< i32, LibError > {
    match item {
      Some(x) => match i32::from_str_radix(x, 10) {
        Ok(result) => Ok::< i32, LibError >(result),
        Err(e) => Err(LibError::ParseIntError(e))
      },
      None => Err(LibError::EntryMissing)
    }
  }
}

impl FromStr for Node {
  type Err = LibError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut splited = s.split(", ");
    let id = try!(Node::parse_number(splited.next())) as u32;
    let x = try!(Node::parse_number(splited.next()));
    let y = try!(Node::parse_number(splited.next()));
    match splited.next() {
      Some(name) => Ok::< Self, Self::Err >(Node::new(name, id, x, y)),
      None => Err(LibError::EntryMissing)
    }
  }
}