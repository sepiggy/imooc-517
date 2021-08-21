fn main() {
    let s2: String;
    
    {
        let s1 = String::from("Hello World");
        s2 = s1;
        // println!("{}", s2);
    }
    
    println!("{}", s2);
}
