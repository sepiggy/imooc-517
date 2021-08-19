fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("max is {}", max(100, 200));
    println!("max is {}", max(10.0, 20.0));
    println!("max is {}", max(10.1f64, 20.1f64));
}
