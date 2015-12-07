extern crate roadsearch;

use roadsearch::map::Node;

#[test]
fn parse_test() {
  let expected = Node::new("Praha", 150, 60);
  let instance = Node::parse("Praha 150 60");
  assert_eq!(expected, instance);
}

#[test]
#[should_panic]
fn parse_failed_test() {
  Node::parse("Praha abc 60");
}

#[test]
fn distance_test() {
  let node1 = Node::new("Praha", 160, 60);
  let node2 = Node::new("Brno", 200, 30);
  let node3 = Node::new("Olomouc", 160, 30);
  assert_eq!(50 as f32, node1.distance(&node2));
  assert_eq!(0 as f32, node1.distance(&node1));
  assert_eq!(30 as f32, node1.distance(&node3));
  assert_eq!(40 as f32, node2.distance(&node3));
}