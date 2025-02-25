fn sorting_1(v: Vec<i32>) -> Vec<i32> {
    Vec::new()
}
#[cfg(test)]
mod sorting_1_tests {
    use super::*;

    #[test]
    fn sort_empty() {
        assert_eq!(sorting_1(Vec::new()), Vec::new());
    }
}
