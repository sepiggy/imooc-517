use mod1::mod2::call;

fn function() {
    println!("function in crate");
}

mod mod1 {
    pub fn function() {
        super::function();
        println!("function in mod1");
    }

    pub mod mod2 {
        pub fn function() {
            println!("function in mod2");
        }

        pub fn call() {
            super::function();
        }
    }
}

fn main() {
    // mod1::function();
    // mod1::mod2::function();
    mod1::mod2::call();
}
