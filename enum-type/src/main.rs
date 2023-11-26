enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrData {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrData::V4(127, 0, 0, 1);

    let loopback = IpAddrData::V6(String::from("::1"));
}
