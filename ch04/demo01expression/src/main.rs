fn main() {
    let cond = true;
    let a = if cond { 42 } else { 24 };
    println!("a={}", a);

    let mut s = 0;
    let mut n = 10;
    let a = loop {
        if n < 0 {
            break s;
        }
        s += n;
        n -= 1;
    };
    println!("{:?}", a);
}
