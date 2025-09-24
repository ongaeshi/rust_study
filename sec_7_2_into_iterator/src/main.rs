// // ケース1 - Vec<T, A> 型の値
// fn main() {
//     let vv = vec![1, 2, 3, 4];
//     let mut iter = vv.into_iter();

//     let x = iter.next().unwrap();
//     println!("{}", x);

//     let x = iter.next().unwrap();
//     println!("{}", x);

//     // let x = vv[2]; // borrow of moved value: `vv`
//     // println!("{}", x);
// }

// // ケース2 - イミュータブルなリファレンス
// fn main() {
//     let vv = vec![1, 2, 3, 4];
//     let mut iter = (&vv).into_iter();

//     let x = iter.next().unwrap();
//     println!("{}", *x); // MEMO: リファレンスであることを明示するためにデリファレンスしているが実際は不要。

//     let x = iter.next().unwrap();
//     println!("{}", *x);

//     let x = vv[2];
//     println!("{}", x);
// }

// ケース3 - ミュータブルなリファレンス
fn main() {
    let mut vv = vec![1, 2, 3, 4];
    let mut iter = (&mut vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x); // MEMO: リファレンスであることを明示するためにデリファレンスしているが実際は不要。

    let x = iter.next().unwrap();
    println!("{}", *x);

    *x += 10; // vv[1] += 10
    println!("{:?}", vv);
}
