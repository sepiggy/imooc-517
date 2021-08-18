enum IPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
    ),
}

fn main() {
    let localhost = IPAddr::IPv4(127, 0, 0, 1);

    match localhost {
        IPAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        _ => {}
    }
}
