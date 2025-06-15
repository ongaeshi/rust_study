mod module_hello;
use crate::module_hello::module_b;
use crate::module_hello::module_b::func_b;
use serde_json::json;

use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        [
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        },
        {
            "name": "ケンジ",
            "age": 22,
            "phones": [
                "+88 3333333"
            ]
        }]"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v[1]["name"], v[1]["phones"][0]);

    Ok(())
}

fn main() {
    module_hello::print_hello();
    module_hello::module_b::func_b();
    module_b::func_b();
    func_b();
    untyped_example();
}
