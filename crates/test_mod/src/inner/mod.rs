pub mod math;
pub use math::add;
use math::inner;

#[allow(dead_code)]
pub fn hello() {
    inner::sub(1, 2);
}
