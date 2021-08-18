const A_CONST: i32 = 42;

fn get_number() -> i32 {
    42
}

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    println!("The value of A_CONST is {}", A_CONST);

    let r = get_number();
    println!("The value of r is {}", r);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
    
    let t = true;
    println!("The value of b is {}", t);
    
    let c = 'c';
    println!("The value of c is {}", c);
}
