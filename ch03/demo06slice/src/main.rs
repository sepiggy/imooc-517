fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3]; // range
    println!("slice[0]={}, len={}", slice[0], slice.len());
    let slice = &arr[3..5];
    println!("slice[0]={}, len={}", slice[0], slice.len());
    println!("{}", slice.len());
    println!("{}", slice.is_empty());

    let slice = &mut arr[..];
    slice[0] = 100000;
    println!("arr[0]={}", arr[0]);
}
