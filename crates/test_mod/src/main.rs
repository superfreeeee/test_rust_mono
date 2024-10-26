mod inner;
mod other;

use crate::inner::math;
use inner::add;

fn main() {
    println!("Hello, world!");
    println!("{}", inner::add(1, 2));
    println!("{}", add(1, 2));
    println!("{}", math::add(1, 2));
}
