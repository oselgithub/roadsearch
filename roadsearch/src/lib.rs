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
  
  fn parse_edge(line: &str) -> Result< Option< (u32, u32) >, LibError > {
    let splitted = line.split(", ").collect::< Vec< _ > >();
    let mut record = splitted.iter().map(|&x| x.parse::< u32 >());
    let from = record.next();
    let to = record.next();
    match from {
      Some(x) => match to {
        Some(y) => Ok(Some((try!(x), try!(y)))),
        None => Ok(None) 
      },
      None => Ok(None)
    }
  }
  
  fn parse_edges(edge_lines: &str) -> Result< Matrix, LibError > {
    let mut  incidency_matrix: Matrix = Matrix::new();
    for line in edge_lines.split("\n") {
      match try!(RoadMap::parse_edge(line)) {
      	Some(res) => incidency_matrix.insert(res.0, res.1),
      	None => break
      };
    }
    Ok(incidency_matrix)
  }
  
  fn parse_nodes(node_lines: &str) -> Result< Vec< Node >, LibError > {
    let mut nodes: Vec< Node > = Vec::new();
    for line in node_lines.split("\n") {
      nodes.push(try!(Node::from_str(line)));
    }
    Ok(nodes)
  }
}

impl FromStr for RoadMap {
  type Err = LibError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.find("\n\n") {
      Some(empty) => Ok::<Self, Self::Err>(RoadMap {
          nodes: try!(RoadMap::parse_nodes(s.split_at(empty).0)),
          incidency_matrix: try!(RoadMap::parse_edges(s.split_at(empty + 2).1)),
        }),
      None => Err(LibError::EntryMissing)
    }
  }
}

} // mod map
