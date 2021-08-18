// loop aka while true
fn main() {
    let mut sum = 0;
    let mut n = 1;
    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break;
        }
    }
    println!("sum is {}", sum);

    // break可以带返回值, 用于简化重试逻辑的代码
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);
}
