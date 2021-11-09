use rust_guide::adder;

#[test]
fn test_add() {
    assert_eq!(adder::add(1, 1), 2);
}