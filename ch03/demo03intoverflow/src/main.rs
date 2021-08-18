fn avg(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}

fn main() {
    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed");

    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 1;

    // let sum = a + b;
    // println!("sum = {}", sum);
    let (sum, is_overflow) = a.overflowing_add(b);
    println!("sum = {:?}, is_overflow = {:?}", sum, is_overflow);
}
