mod module_hello;

fn main() {
    module_hello::print_hello();
    module_hello::module_b::func_b();
}
