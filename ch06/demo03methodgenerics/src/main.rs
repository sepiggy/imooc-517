struct Point<T> {
    x: T,
    y: T,
}

impl<T: Clone + std::cmp::PartialOrd> Point<T> {
    fn max(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        } else {
            self.y.clone()
        }
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let point = Point { x: 10, y: 20 };
    println!("{:?}", point.max());
    let point = Point {
        x: 10.0f32,
        y: 20.0f32,
    };
    println!("{:?}", point.dist_from_origin());
}
