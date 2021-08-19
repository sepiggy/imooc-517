#[derive(Debug)]
enum Alphabet {
    A,
    B,
}

fn main() {
    let mut letter = Alphabet::A;

    while let Alphabet::A = letter {
        println!("{:?}", letter);
        letter = Alphabet::B;
    }
}
