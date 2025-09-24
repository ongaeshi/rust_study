fn main() {
    let mut r = 1..3;

    println!("{:?}", r.next()); // Some(1)
    println!("{:?}", r.next()); // Some(2)
    println!("{:?}", r.next()); // None

    let r = 1..3;
    for i in r {
        println!("{:?}", i); // 1, 2
    }
}
