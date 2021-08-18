use std::mem;

fn main() {
    let a: i8 = -10;
    let b = a as u8;
    println!("a={} b={}", a, b);

    unsafe {
        let a = [0u8, 1u8, 0u8, 0u8];
        let b: u32 = mem::transmute(a);
        println!("{}", b);
    }
}
