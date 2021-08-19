mod mod1 {
    pub const MESSAGE: &str = "Hello";

    pub(crate) enum Enum {
        Item = 4,
    }

    pub(self) const NUMBER: u32 = 42;

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello again";

        pub fn say() {
            println!("{}", super::NUMBER);
        }
    }
}

fn main() {
    println!("{}", mod1::MESSAGE);
    println!("{}", mod1::mod2::MESSAGE);
    println!("{}", mod1::Enum::Item as u32);
}
