mod mod1 {
    pub struct Person {
        pub name: String,
        nickname: String,
    }

    impl Person {
        pub fn new(name: &str, nickname: &str) -> Self {
            Person {
                name: (String::from(name)),
                nickname: (String::from(nickname)),
            }
        }
        pub fn say_nickname(&self) {
            println!("{}", self.nickname);
        }
    }
}

fn main() {
    let p = mod1::Person::new("jack", "baby");
    println!("{}", p.name);
    // println!("{}", p.nickname);
    p.say_nickname();
}
