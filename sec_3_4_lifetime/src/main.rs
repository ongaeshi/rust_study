// fn return_hello() -> &String { // missing lifetime specifier
//     let s = "Hello.".to_string();
//     &s
// }

// fn main() {
//     let s = return_hello();
//     println!("{}", s);
// }

// fn pick1(x: &[i32], end: usize)->&[i32]{
//     &x[..end]
// }

// fn main(){
//     let v1 = [1, 2, 3, 4, 5];
//     let p = pick1(&v1, 2);
//     for ss in p {
//         println!("{}", ss);
//     }
// }

// // tuple で返している。
// // fn pick2(x: &[i32], y: &[i32], end: usize) -> (&[i32], &[i32]) { // error[E0106]: missing lifetime specifiers
// fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'b [i32], &'a [i32]) {
//     (&x[..end], &y[..end])
// }

// fn main() {
//     let v1 = [1, 2, 3, 4, 5];
//     let v2 = [6, 7, 8];

//     let p = pick2(&v1, &v2, 2);
//     for ss in p.0 {
//         println!("{}", ss);
//     }
//     for ss in p.1 {
//         println!("{}", ss);
//     }
// }

// fn incr_by_owned(x: i32) -> i32 {
//     x + 1
// }

// fn incr_by_ref(x_ref: & mut i32) -> i32 {
//     *x_ref += 1;
//     *x_ref
// }

// fn main() {
//     let mut x = 1;
//     let x_ref = & mut x;
//     println!("{}", incr_by_ref(x_ref));
//     println!("{}", incr_by_owned(x));
// }

fn main() {
    let x = 1;
    println!("{}", x);

    let x = 2;
    println!("{}", x);
}