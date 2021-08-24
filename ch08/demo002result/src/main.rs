fn main() {
    // let a: Result<u32, &'static str> = Result::Ok(1);
    // println!("{:?}", a);

    // let b: Result<u32, &'static str> = Result::Err("result error");
    // println!("{:?}", b);

    let r = std::fs::read("/tmp/foo");
    match r {
        Ok(data) => println!("{:?}", std::str::from_utf8(&data).unwrap()),
        Err(err) => println!("{:?}", err),
    }
}
