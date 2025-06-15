mod module_hello;
use crate::module_hello::module_b;
use crate::module_hello::module_b::func_b;

fn main() {
    module_hello::print_hello();
    module_hello::module_b::func_b();
    module_b::func_b();
    func_b();
}
