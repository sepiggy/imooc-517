fn main() {
    let a: i32 = 18;
    let b: char = 'A';

    // tuple
    let t = (a, b);

    // pattern matching
    let (c, d) = t;
    println!("c={}, d={}", c, d);

    println!("t.0={}", t.0);
    println!("t.1={}", t.1);
    
    let (result, is_overflow)  = a.overflowing_add(10);
    println!("result={}, is_overflow={}", result, is_overflow);
    
}
