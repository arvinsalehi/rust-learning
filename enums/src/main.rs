enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}
fn main() {
    let four = IpAddrKind::V4;
    let home = IpAddrKind::V4(String.from("127.0.0.1"));
}
