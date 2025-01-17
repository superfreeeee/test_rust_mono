#[test]
fn test_rc() {
    use std::rc::Rc;

    let a = Rc::new(1);
    let b = a.clone();
    println!("a = {}", a);
    println!("a.count = {}", Rc::strong_count(&a));
    println!("b = {}", b);
    println!("*b = {}", *b);
    println!("b.count = {}", Rc::strong_count(&b));
}
