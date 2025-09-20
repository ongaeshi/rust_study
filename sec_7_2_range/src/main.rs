fn main() {
    let mut r = 1..3;

    println!("{:?}", r.next());
    println!("{:?}", r.next());
    println!("{:?}", r.next());

    let r = 1..3;
    for i in r {
        println!("{:?}", i);
    }
}
