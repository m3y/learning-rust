mod module_c;
mod module_d;

use self::module_c::c;
use super::module_b::module_d::d;

pub fn b() {
    println!("module_b");
    c();
    d();
}
