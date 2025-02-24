struct Stack {
    elements: Vec<i32>,
}
impl Stack {
    fn new() -> Stack {
        Stack {
            elements: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn push(&mut self, value: i32) {
        self.elements.push(value);
    }

    fn pop(&mut self) -> i32 {
        self.elements.pop().expect("Cannot pop from empty stack")
    }

    fn get_size(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn can_create() {
        let s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.get_size(), 0);
    }

    #[test]
    fn after_one_push_is_not_empty() {
        let mut s = Stack::new();
        s.push(1);
        assert!(!s.is_empty());
        assert_eq!(s.get_size(), 1);
    }

    #[test]
    fn after_one_push_and_one_pop_is_empty() {
        let mut s = Stack::new();
        s.push(1);
        s.pop();
        assert!(s.is_empty());
        assert_eq!(s.get_size(), 0);
    }

    #[test]
    fn after_two_pushes_size_is_two() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.get_size(), 2);
    }

    #[test]
    #[should_panic(expected = "Cannot pop from empty stack")]
    fn test_pop_empty_stack() {
        let mut s = Stack::new();
        s.pop(); // This should panic
    }

    #[test]
    fn after_push_will_pop_x() {
        let mut s = Stack::new();
        s.push(1);
        assert_eq!(s.pop(), 1);
        s.push(2);
        assert_eq!(s.pop(), 2);
    }

    #[test]
    fn after_push_x_y_will_pop_y_then_x() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 1);
    }
}
