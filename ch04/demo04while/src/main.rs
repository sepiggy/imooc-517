fn main() {
    let mut n = 1;
    while n < 101 {
        if n % 3 == 0 && n % 5 == 0 {
            print!("fizzbuzz ");
        } else if n % 3 == 0 {
            print!("fizz ");
        } else if n % 5 == 0 {
            print!("buzz ");
        } else {
            print!("{} ", n);
        }
        n += 1;
    }
}
