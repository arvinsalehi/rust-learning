use core::fmt;


enum PpSize {
    Und,    // Undefiend
    L12,    // less than 12 cm
    B12t16, // Between 12 - 16 cm
    M16,    // More than 16 (Considered Big Boy)
}

impl fmt::Display for PpSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PpSize::Und => write!(f, "Undefined"),
            PpSize::L12 => write!(f, "Less than 12 cm"),
            PpSize::B12t16 => write!(f, "Between 12 - 16 cm"),
            PpSize::M16 => write!(f, "More than 16 cm (Considered Big Boy)"),
        }
    }
}
enum MyFriends {
    Arvin(PpSize),
    Arman(PpSize),
    Samin,
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

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

fn eval_friend_pp(friend: MyFriends) {
    /// Give friend a propper PP based on their personality
    /// # Args
    ///
    /// * `friend` - Duh !!!
    match friend {
        MyFriends::Arvin(ppSize) => println!(
            "you have been given {} based on the person you are Congrats",
            ppSize
        ),
        MyFriends::Arman(ppSize) => println!(
            "you have been given {} based on the person you are Congrats",
            ppSize
        ),
        MyFriends::Samin => println!("you don't have pp"),
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

    let my_friend: MyFriends = MyFriends::Arvin(PpSize::B12t16);

    eval_friend_pp(my_friend);

}

// enum IpAddrKind {
//     V4(i32,i32,i32, i32),
// }

// struct IpAddr {
//     kind:IpAddrKind,
//     name: String
// }
// fn main() {
//     let home_ip_addr = IpAddrKind::V4(127, 0, 0, 1);

// }

// fn route(ipaddr: Option<IpAddr>) -> Option<IpAddr>{
//     match ipaddr {
//         Some(ipaddr) => Some(ipaddr),
//         (_) => None,
//     }
// }