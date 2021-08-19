fn fib(n:u64) -> u64 {
    match n  {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}

fn main() {
    println!("fib(5) = {}", fib(5));
    println!("fib(10) = {}", fib(10));
}
