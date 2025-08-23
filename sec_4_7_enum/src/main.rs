enum Sign {
    Positive,
    Zero,
    Negative
}

fn determine_sign(x: i32) -> Sign {
    if x > 0 {
        Sign::Positive
    } else if x < 0 {
        Sign::Negative
    } else {
        Sign::Zero
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Zero => println!("0"),
        Sign::Negative => println!("-"),
    }
}

fn main() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}
