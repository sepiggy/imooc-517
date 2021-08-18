fn main() {
    // 基本用法
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    // 使用for-range语法遍历数组
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in array.iter() {
        print!("{:?} ", i);
    }
    println!();

    for i in array.iter_mut() {
        *i *= 2;
    }
    for i in array.iter() {
        print!("{:?} ", i);
    }
}
