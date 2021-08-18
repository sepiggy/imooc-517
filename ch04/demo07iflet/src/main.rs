enum Alphabet {
    A,
    B,
    C,
}

enum Symbol {
    Char(char),
    Number,
}

fn main() {
    let letter = Alphabet::A;

    if let Alphabet::A = letter {
        println!("It's A");
    }

    // let symbol = Symbol::Char('A');
    let symbol = Symbol::Number;

    if let Symbol::Char(x) = symbol {
        println!("{}", x);
    }
}
