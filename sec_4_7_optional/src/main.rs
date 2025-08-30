fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    match y {
        0 => None,
        _ => Some(x / y),
    }
}

// fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
//     if let Some(x) = ans {
//         println!("{}", x)
//     } else {
//         println!("None")
//     }
// }

fn func_ex_unwrap(x: i32) -> Option<i32> {
    if x >= 0 {
        Some(x)
    } else {
        None
    }
}

fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if ans.is_some() {
        println!("{}", ans.unwrap())
    } else {
        println!("None")
    }
}
 
fn main() {
    func_ex_print_some(func_ex_div_some(4, 2));
    func_ex_print_some(func_ex_div_some(5, 2));
    func_ex_print_some(func_ex_div_some(5, 0));

    println!("{}", func_ex_unwrap(5).unwrap());

    // thread 'main' panicked at src/main.rs:30:39:
    // called `Option::unwrap()` on a `None` value
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //println!("{}", func_ex_unwrap(-1).unwrap());
    println!("{}", func_ex_unwrap(-1).expect("panic panic!!"));
    println!("{}", func_ex_unwrap(-5).unwrap_or(-1));
}
