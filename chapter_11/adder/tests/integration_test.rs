#[test]
fn integ_add_two() {
    let result = adder::add(2, 2);
    assert_eq!(result, 4);
}
