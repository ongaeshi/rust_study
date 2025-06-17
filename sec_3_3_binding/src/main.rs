// fn main() {
//     let s = 1;
//     let t = s;
//     println!("{}", s);
//     println!("{}", t);
// }

// fn main() {
//     let s = "Hello".to_string();
//     let t = s; // error[E0308]: mismatched types
//     println!("{}", s);
//     println!("{}", t);
// }

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    // let s = 1; // OK
    let s = "Hello".to_string();
    let ss = "Hello".to_string();
    myprint(s); 
    // myprint(s); // use of moved value: `s`
    myprint(ss); 
}