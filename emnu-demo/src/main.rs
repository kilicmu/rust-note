

#[derive(Debug)]
enum IpAddrKind {
    v4(u8, u8, u8, u8),
    v6,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) -> i32 {
        println!("{:#?}", self);
        1
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route (_ip_addr: &IpAddr) {
    
}

fn main() {
    let ip_addr = IpAddr {
        kind: IpAddrKind::v4(255, 255, 255, 0),
        address: String::from("127.0.0.1"),
    };
    let ip_addr2 = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("127.0.0.1"),
    };

    let mut opt = Some(String::from("hello"));
    // let mut opt = Option::None;
    match opt.as_mut() {
        Some(v) => *v = String::from("new String"),
        None => {}
    }

    if let IpAddrKind::v4(s1,s2,s3,s4) = ip_addr.kind {
        println!("{}{}{}{}", s1,s2,s3,s4)
    }
    
    println!("{}", opt.unwrap());
    let ret = Message::Write(String::from("hello"));
    ret.call();
    
    route(&ip_addr);
    println!("{:#?}", ip_addr);
    println!("Hello, world!");
}
