#[macro_export]
#[allow(unused_macros)]
macro_rules! inc {
    ($x: expr) => {
        $x + 1
    };
}

#[test]
fn test_inc() {
    println!("inc(1) = {}", inc!(1));
    let _ = inc!(1);
}
