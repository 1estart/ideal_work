fn string_line_wrapper(s: String, w: usize) -> String {
    if s.len() <= w {
        s
    } else {
        let adjusted_with_space = find_last_space_before(&s, w).unwrap_or(w);
        let mut left = s;
        let mut right = left.split_off(adjusted_with_space);
        left = left.trim_end().to_string();
        right = right.trim_start().to_string();
        left + "\n" + &string_line_wrapper(right, w)
    }
}

fn find_last_space_before(s: &str, max_width: usize) -> Option<usize> {
    if s.len() <= max_width {
        None
    } else {
        s[..max_width].rfind(' ')
    }
}

fn assert_wrapper(s: &str, w: usize, expected: &str) {
    assert_eq!(string_line_wrapper(s.to_string(), w), expected.to_string())
}

#[cfg(test)]
mod string_line_wrapper_tests {
    use super::*;

    // #[test]
    // fn test_string_line_wrapper_simple_case() {
    //     assert_wrapper("Four", 7, "Four");
    // }

    // #[test]
    // fn test_with_one_slash_n() {
    //     assert_wrapper("Four score", 7, "Four\nscore");
    // }

    // #[test]
    // fn test_with_one_space() {
    //     assert_wrapper("ago our", 7, "ago our");
    // }

    // #[test]
    // fn test_big_sentence() {
    //     assert_wrapper(
    //         "Four score and seven years ago our",
    //         7,
    //         "Four\nscore\nand\nseven\nyears\nago our",
    //     );
    // }

    #[test]
    fn test_empty_string() {
        assert_wrapper("", 1, "");
    }

    #[test]
    fn test_one_symbol() {
        assert_wrapper("a", 1, "a");
    }

    #[test]
    fn test_two_symbols() {
        assert_wrapper("xx", 1, "x\nx");
        assert_wrapper("x x", 1, "x\nx");
    }

    #[test]
    fn test_two_symbols_with_two() {
        assert_wrapper("xx", 2, "xx");
    }

    #[test]
    fn test_three_symbols() {
        assert_wrapper("xxx", 1, "x\nx\nx");
        assert_wrapper("xxx", 2, "xx\nx");
        assert_wrapper("xxx", 3, "xxx");
    }

    #[test]
    fn test_with_space_inside() {
        assert_wrapper("xx xxx", 4, "xx\nxxx");
    }

    #[test]
    fn test_from_book() {
        assert_wrapper(
            "Four score and seven years ago out fathers brought force",
            15,
            "Four score and\nseven years\nago out\nfathers\nbrought force",
        );
    }
}
