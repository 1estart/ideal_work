struct Stack {
    empty: bool,
}
impl Stack {
    fn new() -> Stack {
        Stack { empty: true }
    }

    fn is_empty(&self) -> bool {
        self.empty
    }

    fn push(&mut self, value: i32) {
        self.empty = false;
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn can_create() {
        let s = Stack::new();
        assert!(s.is_empty())
    }

    #[test]
    fn after_one_push_is_not_empty() {
        let mut s = Stack::new();
        s.push(1);
        assert!(!s.is_empty());
    }
}
