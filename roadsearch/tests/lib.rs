extern crate roadsearch;

use std::str::FromStr;
use roadsearch::map::{Matrix, Node, RoadMap};

#[test]
fn parse_test() {
  let expected = Node::new("Praha", 1, 150, 60);
  let instance = Node::from_str("1, 150, 60, Praha").unwrap();
  assert_eq!(expected, instance);
}

#[test]
fn parse_failed_test() {
  assert_eq!(true, Node::from_str("Praha abc 60").is_err());
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
  let instance = RoadMap::from_str(
    "1, 160, 60, Praha\n2, 200, 30, Brno\n3, 160, 30, Olomouc\n\n1, 2, 65\n1, 3, 45\n3, 2, 88");
  assert_eq!(expected, instance.unwrap());
}

#[test]
fn roadmap_failed_test() {
  let instance1 = RoadMap::from_str(
    "1, abc, 60, Praha\n2, 200, 30, Brno\n3, 160, 30, Olomouc\n\n1, 2, 65\n1, 3, 45\n3, 2, 88");
  assert_eq!(true, instance1.is_err());
  let instance2 = RoadMap::from_str(
    "1, 160, 60, Praha\n2, 200, 30, Brno\n3, 160, 30, Olomouc\n\n1, 2, 65\n1, xyz, 45\n3, 2, 88");
  assert_eq!(true, instance2.is_err());
}
