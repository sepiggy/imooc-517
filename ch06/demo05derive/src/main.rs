#[derive(Debug, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 10, y: 20 };
    println!("{:?}", p1);
    println!("{:?}", p1 == p2);
    
    let p = Point::default();
    println!("{:?}", p);
}
