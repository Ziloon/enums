#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}

    fn build() -> Message {
        Message::Quit
    }
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
    q.call();

    let msg = match q {
        Message::Move { x, y } => m,
        Message::ChangeColor(r,g ,b ) => c,
        Message::Write(x) => w,
        _ => Message::build(),
    };
    println!("{:?}", msg);

    // Option
    let x = 5;
    let y = Some(5);
    assert_eq!(y.is_some(), true);

}

fn route(ip_kind: IpAddrKind) {
    println!("route to {:#?}", ip_kind);
}
