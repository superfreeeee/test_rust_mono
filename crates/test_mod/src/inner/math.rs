pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

pub mod inner {
    #[allow(dead_code)]
    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }
}

#[cfg(test)]
mod math_tests {
    use crate::inner::add;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
