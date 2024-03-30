#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);

    let q = Message::Quit;
    let _m = Message::Move { x: 12, y: 24 };
    let _w = Message::Write(String::from("Hello"));
    let _c = Message::ChangeColor(0, 255, 255);
    q.call();
}

fn route(ip_kind: IpAddrKind) {
    println!("route to {:#?}", ip_kind);
}
