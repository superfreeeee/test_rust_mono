pub mod math;
pub use math::add;
use math::inner;

pub fn hello() {
    inner::sub(1, 2);
}
