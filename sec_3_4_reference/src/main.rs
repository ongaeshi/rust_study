// fn myprint<T: std::fmt::Display>(msg: &T) {
//     // 厳密にはデリファレンスが必要だが println! の第2引数以降は省略可能。
//     println!("{}", *msg);
// }

fn myclear(x: &mut String) {
    x.clear();
}

// fn main() {
//     let s = "Hello".to_string();
//     myprint(&s);
//     myprint(&s); 
// }

// fn main() {
//     let s = "Hello".to_string();
//     let s_ref = &s;
//     myprint(s_ref);
//     myprint(s_ref); 
// }

// fn main() {
//     let mut s = "Hello".to_string();
//     println!("s= {}", s);

//     let s_ref = &mut s;
//     let s_ref2 = &mut s;
//     myclear(s_ref);
//     println!("s= {}", s_ref2);
// }

// fn main() {
//     let x = 1;
//     println!("{:p}", &x);
// }

// fn main() {
//     let mut s = "Hello".to_string();
//     println!("s= {}", s);

//     let s_ref = &mut s;
//     myclear(s_ref);
//     println!("s= {}", s_ref);

//     let s_ref2 = &mut s;
//     myclear(s_ref2);
//     println!("s= {}", s_ref2);
// }

fn main {
    let mut x = 1;
    let x_ref = &x;

    x = 2; // cannot assign to `x` because it is borrowed
    println!("{}", x_ref);
}