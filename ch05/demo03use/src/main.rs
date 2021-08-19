use std::fs as stdfs;

fn main() {
    let data = stdfs::read("src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());
}
