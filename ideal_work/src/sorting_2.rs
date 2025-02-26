fn sorting_2(v: Vec<i32>) -> Vec<i32> {
    v
}

#[cfg(test)]
mod sorting_2_tests {
    use super::*;

    #[test]
    fn sorting_2_simple_test() {
        assert_eq!(sorting_2(vec![]), vec![]);
        assert_eq!(sorting_2(vec![1]), vec![1]);
        assert_eq!(sorting_2(vec![1, 2]), vec![1, 2]);
    }

    #[test]
    fn minimal_two_unsorted() {
        assert_eq!(sorting_2(vec![2, 1]), vec![1, 2]);
    }
}
