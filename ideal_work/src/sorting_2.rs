use std::vec;

fn merge_with_middle(vec1: Vec<i32>, middle: i32, vec2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(vec1.len() + vec2.len() + 1);
    result.extend(vec1);
    result.push(middle);
    result.extend(vec2);
    result
}

fn sorting_2(v: Vec<i32>) -> Vec<i32> {
    let n = v.len();
    if n < 1 {
        v
    } else {
        let mut lessers = Vec::new();
        let mut greaters = Vec::new();
        let pivot = v[0];
        for i in 1..n {
            if v[i] < pivot {
                lessers.push(v[i]);
            } else {
                greaters.push(v[i]);
            }
        }
        merge_with_middle(sorting_2(lessers), pivot, sorting_2(greaters))
    }
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

    #[test]
    fn minimal_three_unsorted() {
        assert_eq!(sorting_2(vec![3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(sorting_2(vec![3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(sorting_2(vec![1, 3, 2]), vec![1, 2, 3]);
        assert_eq!(sorting_2(vec![2, 1, 3]), vec![1, 2, 3]);
        assert_eq!(sorting_2(vec![3, 1, 2]), vec![1, 2, 3]);
        assert_eq!(sorting_2(vec![2, 3, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn four_sorted() {
        assert_eq!(sorting_2(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(sorting_2(vec![1, 2, 4, 3]), vec![1, 2, 3, 4]);
        assert_eq!(sorting_2(vec![1, 3, 1, 4]), vec![1, 1, 3, 4]);
    }
}
