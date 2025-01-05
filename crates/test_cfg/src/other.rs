#[macro_export]
macro_rules! debug_call {
    ($expr:expr) => {
        #[cfg(debug_assertions)]
        $expr
    };
}
