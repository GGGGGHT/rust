use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// 枚举 模式匹配
enum Color {
    Red,
    Green,
    Blue,
}

// 这个枚举有四个含有不同类型的成员：

// Quit 没有关联任何数据。
// Move 类似结构体包含命名字段。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn process(&self) {
    
    }
}
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体


// #[derive(Debug)]
// enum IpAddrKind {
//     V4(&'a str),
//     V6(&'a str),
// }
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn route(ip_kind: IpAddrKind) {}

fn main() {
    println!("Hello, world!");

    let m = Message::Write(String::from("Hello from other side"));

    m.process();

    {
        // option
        let some_number = Some(5);

        let some_char = Some('e');

        let absent_number : Option<u32> = None;

    }
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(four);
    // println!("four: {:#?}", six);

    // let home = IpAddr {
    // kind: IpAddrKind::V4,
    // address: String::from("127.0.0.1"),
    // };

    // let home = IpAddrKind::V4("127.0.0.1");
    // let lb = IpAddrKind::V6("::1");

    // let loopback = IpAddr {
    // kind: IpAddrKind::V6,
    // address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));
    // dbg!(&home);
    // dbg!(&loopback);

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));

    assert_eq!(localhost_v4.is_ipv6(), false);
    assert_eq!(localhost_v4.is_ipv4(), true);
}
