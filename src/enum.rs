enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

// pay attention to the difference
enum IpAddr2 {
    V4(String),
    V6(String),
}

// use u8 to store IPv4 addr
// arbitrary type var can be stored in enum field
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
    A1(IpAddrKind)
}

// use Debug trait to print
#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


