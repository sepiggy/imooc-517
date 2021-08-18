enum Alphabet {
    A,
    B,
}

fn main() {
    let letter = Alphabet::A;

    match letter {
        Alphabet::A => {
            println!("It's A");
        }
        Alphabet::B => {
            println!("It's B");
        }
    }

    let n: u8 = 42;

    match n {
        42 => {
            println!("bingo!");
        }
        _ => {
            println!("{}", n);
        }
    }
}
