#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    // 构造方法
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }

    fn get_x(&self) -> u64 {
        self.x
    }

    fn set_x(&mut self, x: u64) {
        self.x = x
    }
}

fn main() {
    let mut p = Point::new(10, 20);
    println!("{:?}", p);
    println!("p.get_x={}", p.get_x());
    p.set_x(10000);
    println!("{:?}", p);
}
