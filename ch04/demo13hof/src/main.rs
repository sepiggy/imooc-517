type Method = fn(u32, u32) -> u32;

fn calc(method: Method, a: u32, b: u32) -> u32 {
    method(a, b)
}

fn main() {
    let r1 = calc(|a: u32, b: u32| -> u32 { a + b }, 200, 100);
    println!("the add result of a and b is {}", r1);
    let r2 = calc(|a, b| -> u32 { a - b }, 200, 100);
    println!("the minus result of a and b is {}", r2);
}
