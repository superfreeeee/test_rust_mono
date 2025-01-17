#[test]
fn test_ownership_move() {
    let a = 1;
    let b = a;
    println!("a = {}", a);
    println!("b = {}", b);
}

#[test]
fn test_box() {
    let mut num = Box::new(1);
    // get
    println!("num = {}", *num);
    // set
    *num = 2;
    println!("num = {}", *num);
}
