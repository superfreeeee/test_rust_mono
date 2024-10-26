use std::mem;

use memoffset::offset_of;

fn main() {}

struct A {
    a: u8,
}

struct A2 {
    a: u16,
}

struct A3 {
    a: u32,
}

struct A4 {
    a: u64,
}

// 测试 单字段
#[test]
fn test_a() {
    println!("A size = {}", mem::size_of::<A>());
    println!("A2 size = {}", mem::size_of::<A2>());
    println!("A3 size = {}", mem::size_of::<A3>());
    println!("A4 size = {}", mem::size_of::<A4>());

    println!("A align = {}", mem::align_of::<A>());
    println!("A2 align = {}", mem::align_of::<A2>());
    println!("A3 align = {}", mem::align_of::<A3>());
    println!("A4 align = {}", mem::align_of::<A4>());
}

// 3 bytes
struct B {
    a: u8,
    b: u16,
}

struct B2 {
    b: u16,
    a: u8,
}

// 5 bytes
struct B3 {
    a: u8,
    c: u16,
    b: u16,
}

struct B4 {
    c: u16,
    a: u8,
    b: u16,
}

struct B5 {
    c: u16,
    b: u16,
    a: u8,
}

// 7 btyes
struct B6 {
    a: u8,
    b: u16,
    c: u32,
}

struct B7 {
    a: u8,
    c: u32,
    b: u16,
}

struct B8 {
    b: u16,
    a: u8,
    c: u32,
}

struct B9 {
    b: u16,
    c: u32,
    a: u8,
}

struct B10 {
    c: u32,
    a: u8,
    b: u16,
}

struct B11 {
    c: u32,
    b: u16,
    a: u8,
}

struct B12 {
    a: u8,
    b: u8,
    c: u16,
}

// 测试 奇数 bytes 字段 & 字段顺序
#[test]
fn test_b() {
    println!("B size = {}", mem::size_of::<B>());
    println!("B2 size = {}", mem::size_of::<B2>());
    println!("B3 size = {}", mem::size_of::<B3>());
    println!("B4 size = {}", mem::size_of::<B4>());
    println!("B5 size = {}", mem::size_of::<B5>());
    println!("B6 size = {}", mem::size_of::<B6>());
    println!("B7 size = {}", mem::size_of::<B7>());
    println!("B8 size = {}", mem::size_of::<B8>());
    println!("B9 size = {}", mem::size_of::<B9>());
    println!("B10 size = {}", mem::size_of::<B10>());
    println!("B11 size = {}", mem::size_of::<B11>());
    println!("B12 size = {}", mem::size_of::<B12>());

    println!("B align = {}", mem::align_of::<B>());
    println!("B2 align = {}", mem::align_of::<B2>());
    println!("B3 align = {}", mem::align_of::<B3>());
    println!("B4 align = {}", mem::align_of::<B4>());
    println!("B5 align = {}", mem::align_of::<B5>());
    println!("B6 align = {}", mem::align_of::<B6>());
    println!("B7 align = {}", mem::align_of::<B7>());
    println!("B8 align = {}", mem::align_of::<B8>());
    println!("B9 align = {}", mem::align_of::<B9>());
    println!("B10 align = {}", mem::align_of::<B10>());
    println!("B11 align = {}", mem::align_of::<B11>());
    println!("B12 align = {}", mem::align_of::<B12>());

    println!(
        "B6 offset, c = {}, b = {}, a = {}",
        offset_of!(B6, c),
        offset_of!(B6, b),
        offset_of!(B6, a)
    );
    println!(
        "B8 offset, c = {}, b = {}, a = {}",
        offset_of!(B8, c),
        offset_of!(B8, b),
        offset_of!(B8, a)
    );
    println!(
        "B11 offset, c = {}, b = {}, a = {}",
        offset_of!(B11, c),
        offset_of!(B11, b),
        offset_of!(B11, a)
    );
    println!(
        "B12 offset, c = {}, a = {}, b = {}",
        offset_of!(B12, c),
        offset_of!(B12, a),
        offset_of!(B12, b),
    );
}
