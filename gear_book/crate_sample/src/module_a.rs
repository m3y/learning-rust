use crate::module_b;

pub fn a() {
    println!("module_a");
    module_b::b();
}
