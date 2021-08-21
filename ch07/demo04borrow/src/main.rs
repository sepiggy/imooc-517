fn echo(s: &String) {
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(" changed!");
}

fn main() {
    let mut s = String::from("Hello World");
    echo(&s);
    println!("{}", s);
    change(&mut s);
    println!("{}", s);
}
