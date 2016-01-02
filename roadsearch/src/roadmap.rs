use std::collections::HashMap;
use std::str::FromStr;

use error::LibError;
use node::Node;


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
  
  pub fn print_path(&self, start: &str, destination: &str)  {
    //TODO A* to find path
  }

  fn parse_edge(line: &str) -> Result< Option< (u32, u32) >, LibError > {
    let splitted = line.split(", ").collect::< Vec< _ > >();
    let mut record = splitted.iter().map(|&x| x.parse::< u32 >());
    match (record.next(), record.next()) {
      (Some(x), Some(y)) => Ok(Some((try!(x), try!(y)))),
      _ => Ok(None)
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
