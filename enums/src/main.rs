enum IpAddrKind {
    V4(String),
    V6(String),
}

<<<<<<< HEAD
fn route(ip_kind: IpAddrKind) {}
fn main() {
    let four = IpAddrKind::V4;
    let home = IpAddrKind::V4(String.from("127.0.0.1"));
=======
struct IpAddr {
    addr: IpAddrKind,
}

#[derive(Debug)]
enum Message {
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
fn main() {
    let home = IpAddr {
        addr: IpAddrKind::V4(String::from("127.0.0.1")),
    };

    let loop_back: IpAddr = IpAddr {
        addr: IpAddrKind::V6(String::from("::1")),
    };
    match home.addr {
        IpAddrKind::V4(ip) => println!("Home IPv4 is : {}", ip),
        IpAddrKind::V6(ip) => println!("Home IPv6 is : {}", ip),
    }
    match loop_back.addr {
        IpAddrKind::V4(ip) => println!("loop_back IPv4 is : {}", ip),
        IpAddrKind::V6(ip) => println!("loop_back IPv6 is : {}", ip),
    }

    let message: Message = Message::Write(String::from("Fuck you"));
    message.call();
>>>>>>> origin/Enums
}
