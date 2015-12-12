pub mod map {

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

  pub fn parse(line: &str) -> Node {
    let mut splited = line.split(", ");
    Node {
      id: splited.next().unwrap().parse::< u32 >().unwrap(),
      x: splited.next().unwrap().parse::< i32 >().unwrap(),
      y: splited.next().unwrap().parse::< i32 >().unwrap(),
      name: splited.next().unwrap().to_string(),
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
}

use std::collections;
pub type Matrix = collections::HashMap< u32, u32 >;

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
      nodes.push(Node::parse(line));
    }
    for line in edge_lines.split("\n") {
      let mut record = line.split(", ");
      incidency_matrix.insert(
        record.next().unwrap().parse::< u32 >().unwrap(),
        record.next().unwrap().parse::< u32 >().unwrap());
    }
    RoadMap {
      nodes: nodes,
      incidency_matrix: incidency_matrix,
    }
  }
}

} // mod map
