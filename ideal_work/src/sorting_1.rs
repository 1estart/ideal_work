fn sorting_1(v: Vec<i32>) -> Vec<i32> {
    let mut v_mut = v;
    let n = v_mut.len();
    if n > 1 {
        for limit in (0..=n - 2).rev() {
            for i in 0..=limit {
                if v_mut[i] > v_mut[i + 1] {
                    v_mut.swap(i, i + 1);
                }
            }
        }
    }
    v_mut
}
#[cfg(test)]
mod sorting_1_tests {
    use super::*;

    #[test]
    fn sort_empty() {
        assert_eq!(sorting_1(Vec::new()), Vec::new());
    }

    #[test]
    fn sort_with_one_element() {
        assert_eq!(sorting_1(vec![1]), vec![1]);
    }

    #[test]
    fn sort_two_sorted_elements() {
        assert_eq!(sorting_1(vec![1, 2]), vec![1, 2]);
    }

    #[test]
    fn sort_two_unsorted_elements() {
        assert_eq!(sorting_1(vec![2, 1]), vec![1, 2]);
    }

    #[test]
    fn sort_three_sorted_elements() {
        assert_eq!(sorting_1(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn sort_three_unsorted_elements_with_two_first() {
        assert_eq!(sorting_1(vec![2, 1, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn sort_three_unsorted_elements_with_two_last_unsorted() {
        assert_eq!(sorting_1(vec![2, 3, 1]), vec![1, 2, 3]);
    }
}
