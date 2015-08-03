extern crate testing;
//Intergration tests can go here, they act as an external user of the crate
#[test]
fn two_and_two_is_four() {
    assert_eq!(4, testing::add_two(2));
}