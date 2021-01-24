use adder;

mod common;

// integration test of a function in the adder crate
#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}
