/// Represents the kind of IP address, which can be either IPv4 or IPv6.
enum IpAddrKind {
    V4,
    V6,
}

/// Represents an IP address with its kind (IPv4 or IPv6) and address string.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/// Represents the data associated with an IP address, which can be either IPv4 or IPv6.
enum IpAddrData {
    /// IPv4 address represented as four octets.
    V4(u8, u8, u8, u8),
    /// IPv6 address represented as a string.
    V6(String),
}

fn main() {
    // Example of creating an IPv4 address instance.
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // Example of creating an IPv6 address instance.
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Example of creating an IPv4 address using the IpAddrData enum.
    let home = IpAddrData::V4(127, 0, 0, 1);

    // Example of creating an IPv6 address using the IpAddrData enum.
    let loopback = IpAddrData::V6(String::from("::1"));
}
