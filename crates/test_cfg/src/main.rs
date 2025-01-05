use test_macro::test_greet;

mod other;

fn main() {
    println!("Hello, world!");

    // 按 features 条件编译
    #[cfg(feature = "add")]
    {
        println!("with features add");
        println!("{}", add(1, 2));
    }

    debug_call!(debug_log());

    test_greet!();
}

#[cfg(feature = "add")]
fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[cfg(debug_assertions)]
fn debug_log() {
    println!("debug_log on debug_assertions")
}
