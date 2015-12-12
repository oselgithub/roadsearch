pub mod map {

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

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


impl fmt::Display for LibError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl From< ParseIntError > for LibError {
    fn from(err: ParseIntError) -> LibError {
      LibError::ParseIntError(err)
    }
}

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

pub type Matrix = HashMap< u32, u32 >;

#[derive(Debug,Clone,PartialEq)]
pub struct RoadMap {
  nodes: Vec< Node >,
  incidency_matrix: Matrix,
}

impl RoadMap {
  pub fn new(nodes: &Vec< Node >, incidency_matrix: &Matrix) -> RoadMap {
    RoadMap {
      nodes: nodes.clone(),
      incidency_matrix: incidency_matrix.clone(),
    }
  }

  pub fn parse(file: &str) -> RoadMap {
    let empty = file.find("\n\n").unwrap();
    let node_lines = file.split_at(empty).0;
    let edge_lines = file.split_at(empty + 2).1;
    let mut nodes: Vec< Node > = Vec::new();
    let mut  incidency_matrix: Matrix = Matrix::new();
    for line in node_lines.split("\n") {
      nodes.push(Node::from_str(line).unwrap());
    }
    for line in edge_lines.split("\n") {
      match RoadMap::parse_edge(line) {
      	Some(res) => incidency_matrix.insert(res.0, res.1),
      	None => break
      };
    }
    RoadMap {
      nodes: nodes,
      incidency_matrix: incidency_matrix,
    }
  }
  
  fn parse_edge(line: &str) -> Option< (u32, u32) > {
    let splitted = line.split(", ").collect::< Vec< _ > >();
    let mut record = splitted.iter().map(|&x| x.parse::< u32 >().unwrap());
    let from = record.next();
    let to = record.next();
    match from {
      Some(x) => match to {
        Some(y) =>Some((x, y)),
        None => None 
      },
      None => None
    }
  }
}

} // mod map
