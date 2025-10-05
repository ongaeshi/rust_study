const N_MAX: usize = 100_000_000;
const N_THREAD: usize = 4;

const N_ELEM_PER_THRD: usize = N_MAX / N_THREAD;
const RESIDUAL: usize = N_MAX % N_THREAD;

// 複数スレッドで計算
// ベクトルコピー版
// for i in $(seq 3); do time ./sec_8_2_sum_of_vector_elements; done
// 5000000050000000
// real    0m1.212s
// real    0m0.736s
// real    0m0.767s
//
// Arcで共有版
// real    0m0.879s
// real    0m0.534s
// real    0m0.460s
fn main() -> std::thread::Result<()> {
    if RESIDUAL != 0 {
        panic!("invalid combination of N_MAX and N_THREAD");
    }

    let mut thrd = Vec::new();
    // let v = (1..=N_MAX).collect::<Vec<usize>>();
    let v = std::sync::Arc::new((1..=N_MAX).collect::<Vec<usize>>()); // Arcで共有版

    // 1..=N_MAX を N_THREAD に分割して、それぞれの和をスレッドで計算。
    for ii in 0..N_THREAD {
        let ist = ii * N_ELEM_PER_THRD;
        let ien = ist + N_ELEM_PER_THRD;
        // let vv = (&v[ist..ien]).to_owned(); // ベクトルデータをコピーしないとスレッドで使えない
        // let th = std::thread::spawn(move || vv.into_iter().sum::<usize>());
        let vv = std::sync::Arc::clone(&v);
        let th = std::thread::spawn(move || vv[ist..ien].iter().sum::<usize>());
        thrd.push(th);
    }

    // 各スレッドで計算した値を集めて、その和を取り、全体の和を求める
    let ans: usize = thrd
        .into_iter()
        .map(|r| r.join().unwrap())
        .sum::<usize>();
    println!("{}", ans);
    assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
    Ok(())
}

// シングルスレッド版
// real    0m1.565s
// real    0m0.409s
// real    0m0.411s
// fn main() {
//     let v = (1..=N_MAX).collect::<Vec<usize>>();
//     let ans = v[0..N_MAX].iter().sum::<usize>();
//     println!("{}", ans);
//     assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
// }
