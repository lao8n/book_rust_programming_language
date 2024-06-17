use tests_integration::add_two;

mod common;

#[test] // don't annotate with #[cfg(test)]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}