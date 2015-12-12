extern crate roadsearch;

use roadsearch::map::Matrix;
use roadsearch::map::Node;
use roadsearch::map::RoadMap;

#[test]
fn parse_test() {
  let expected = Node::new("Praha", 1, 150, 60);
  let instance = Node::parse("1, 150, 60, Praha");
  assert_eq!(expected, instance);
}

#[test]
#[should_panic]
fn parse_failed_test() {
  Node::parse("Praha abc 60");
}

#[test]
fn distance_test() {
  let node1 = Node::new("Praha", 1, 160, 60);
  let node2 = Node::new("Brno", 2, 200, 30);
  let node3 = Node::new("Olomouc", 3, 160, 30);
  assert_eq!(50 as f32, node1.distance(&node2));
  assert_eq!(0 as f32, node1.distance(&node1));
  assert_eq!(30 as f32, node1.distance(&node3));
  assert_eq!(40 as f32, node2.distance(&node3));
}

#[test]
fn roadmap_test() {
  let mut incidency = Matrix::new();
  incidency.insert(1, 2);
  incidency.insert(1, 3);
  incidency.insert(3, 2);
  let expected = RoadMap::new(&vec![
    Node::new("Praha", 1, 160, 60),
    Node::new("Brno", 2, 200, 30),
    Node::new("Olomouc", 3, 160, 30)],
    &incidency);
  let instance = RoadMap::parse(
    "1, 160, 60, Praha\n2, 200, 30, Brno\n3, 160, 30, Olomouc\n\n1, 2, 65\n1, 3, 45\n3, 2, 88");
  assert_eq!(expected, instance);
}
