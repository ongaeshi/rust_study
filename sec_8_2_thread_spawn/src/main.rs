use std::thread::spawn;

// コピーセマンティクスが適用される場合
// fn main() {
//     let mut v_threads = Vec::new();
//     for i in 0..10 {
//         let thread = spawn(move || { 
//             println!("{}", i);
//         });
//         v_threads.push(thread);
//     }
//     let _x: Vec<()> = v_threads.into_iter().map(|th| th.join().unwrap()).collect();
// }

// ムーブセマンティクスが適用される場合
// スレッドを複数起動するときには、スレッド同士が干渉せずに独立にデータを持てるように必要な値をコピーして渡す必要がある。
// → コピーしないとコンパイルエラーになる。
fn main() {
    let mut v_threads = Vec::new();
    let hello = String::from("Hello");
    for i in 0..10 {
        let hello_cloned = hello.clone();
        let thread = spawn(move ||  println!("{}: {}", i, hello_cloned));
        v_threads.push(thread);
    }
    let _x: Vec<()> = v_threads.into_iter().map(|th| th.join().unwrap()).collect();
}
