fn bar() -> Result<u32, &'static str> {
    Ok(0)
}

fn foo() -> Result<i32, &'static str> {
    let a = bar()?;
    Ok(a as i32)
}

pub enum Error {
    IO(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

fn main() {
    println!("{:?}", foo());
}
