pub mod map {

#[derive(Debug,Clone,PartialEq)]
pub struct Node {
  name: String,
  x: i32,
  y: i32,
}

impl Node {
  pub fn new(name: &str, x: i32, y: i32) -> Node {
    Node {
      name: name.to_string(),
      x: x,
      y: y,
    }
  }
  
  pub fn parse(line: &str) -> Node {
    let mut splited = line.split(" ");
    Node {
      name: splited.next().unwrap().to_string(),
      x: splited.next().unwrap().parse::< i32 >().unwrap(),
      y: splited.next().unwrap().parse::< i32 >().unwrap(),
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

} // mod map