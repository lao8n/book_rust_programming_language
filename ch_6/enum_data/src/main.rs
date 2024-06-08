enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr { // bad way
    kind: IpAddrKind,
    address: String,
}

// rather than enum inside a struct, have data inside the enum
enum IpAddr {
    V4(u8, u8, u8, u8), // IpAddr::V4(127, 0, 0, 1)
    V6(String),
}

// can even embed structs inside enums 
enum IpAddr {
    V4(Ipv4Addr), // struct
    V6(Ipv6Addr), // struct
}