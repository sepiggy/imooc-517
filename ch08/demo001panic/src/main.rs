fn add(a: u32, b: u32) -> u32 {
    unimplemented!();
}

fn divide_by_three(x:u32) -> u32 {
    unreachable!();
}
fn main() {
    // panic!宏
    // panic!("error!");
    // println!("here");

    // 断言
    // assert!(1==2);
    // assert_eq!(1, 2);

    // 未实现的代码
    // add(1,2);

    // 不应当被访问到的代码
    divide_by_three(100);
}
