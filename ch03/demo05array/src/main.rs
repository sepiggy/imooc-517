fn main() {
    let ints: [u32; 5] = [1, 2, 3, 4, 5];
    println!("ints[0]={}", ints[0]);

    // 编译期错误
    // println!("ints[5]={}", ints[5]);

    // 运行期错误
    // let index = "5".parse::<usize>().unwrap();
    // println!("ints[5]={}", ints[index]);

    let mut buffer: [u32; 32 * 1024] = [0; 32 * 1024];
    println!("buffer[1024]={}", buffer[1024]);
    buffer[1024] = 100;
    println!("buffer[1024]={}", buffer[1024]);
}
