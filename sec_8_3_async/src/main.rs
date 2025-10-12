async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).into_iter().sum::<usize>();
    println!("{}", ans);
    ans
}

// fn main() {
//     let ls = tokio::task::LocalSet::new();
//     let rt = tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     // fut の単体実行
//     // let fut = sum_func(10_000_000);
//     // ls.block_on(&rt, fut);

//     ls.block_on(&rt, async {
//         sum_func(10_000_000).await;
//         sum_func(20_000_000).await;
//     });
// }

// ↑と同じ意味
#[tokio::main]
async fn main() {
    sum_func(10_000_000).await;
    sum_func(20_000_000).await;
}