use std::sync::Arc;
use std::thread::spawn;

fn main() {
    let data = Arc::new(Vec::new());

    let added = vec![1, 2, 3, 4, 5];

    let mut thrd = Vec::new();
    for aa in added {
        let data = Arc::clone(&data);
        let th = spawn(move || {
            data.push(aa);
        });
        thrd.push(th);
    }

    thrd.into_iter().for_each(|th| {
        let _ = th.join();
    });

    println!("{:?}", *data);
}
