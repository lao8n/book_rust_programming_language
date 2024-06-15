pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// run with cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // attribute is metadata about test functions - e.g. derive or test here
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // assert_ne! is another macro
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
