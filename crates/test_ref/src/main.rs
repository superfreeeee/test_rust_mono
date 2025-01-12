fn main() {}

#[test]
fn test_map_borrow() {
    use std::collections::HashMap;

    let mut map = HashMap::<&str, u32>::new();
    map.insert("A", 1);
    let v1 = map.get("A").unwrap();
    println!("v1 = {}", v1);
    let _ = &mut map;
    // println!("v1 = {}", v1);
}

#[test]
fn test_lifetime_borrow() {
    struct Context {
        count: u32,
    }

    let mut context = Context { count: 0 };
    let show_count = |c: &Context| {
        println!("count = {}", c.count);
    };

    show_count(&context);
    context.count += 1;
    show_count(&context);

    fn get_count(c: &mut Context) -> &u32 {
        c.count += 1;
        &c.count
    }

    let count = get_count(&mut context);
    // context.count += 1; // ! 直到 count 归还前，不能再使用 &context
    // show_count(&context); // ! 直到 count 归还前，不能再使用 &context
    println!("count = {}", count);
}
