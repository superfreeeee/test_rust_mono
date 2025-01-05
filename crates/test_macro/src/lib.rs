mod macros;

pub mod test {
    pub fn greeting() {
        println!("greeting from macro")
    }

    #[macro_export]
    macro_rules! test_greet {
        () => {
            $crate::test::greeting();
        };
    }
}
