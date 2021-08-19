// 结构体泛型嵌套
struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    x: Point<T>,
    y: Point<T>,
}
fn main() {
    let point1 = Point { x: 0, y: 0 };
    let point2 = Point { x: 2, y: 2 };
    let line = Line {
        x: point1,
        y: point2,
    };

    println!("{} {} {} {}", line.x.x, line.x.y, line.y.x, line.y.y);
}
