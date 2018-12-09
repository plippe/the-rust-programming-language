enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn home() -> IpAddr {
        IpAddr::V4(127, 0, 0, 1)
    }

    fn loopback() -> IpAddr {
        IpAddr::V6(String::from("::1"))
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum Option<T> {
    Some(T),
    None,
}

impl<A> Option<A> {
    fn map<B>(&self, f: &Fn(&A) -> B) -> Option<B> {
        match self {
            Option::Some(value) => Option::Some(f(value)),
            _ => Option::None,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = Option::None;

    let six = Some(5).map(|n| n + 1);
    let none = None.map(|n: i32| n + 1);
}

fn route(ip_type: IpAddrKind) {}
