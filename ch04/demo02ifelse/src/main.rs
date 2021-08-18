fn main() {
    let n = 0;

    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }

    let m = if n < 0 { 2.0 } else { 3.0 };
    println!("m is {}", m);
}
