const BUFSIZE: usize = 1024;
fn main() {
    let mut ibuf = [0i32; BUFSIZE];

    for ii in 0..BUFSIZE + 2 {
        ibuf[ii] = ii as i32;
    }
}
