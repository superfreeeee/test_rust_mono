fn main() {
    println!("Hello, world!");

    // 按 features 条件编译
    #[cfg(feature = "add")]
    {
        println!("with features add");
        println!("{}", add(1, 2));
    }
}

#[cfg(feature = "add")]
fn add(x: u32, y: u32) -> u32 {
    x + y
}
