#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6
}

#[derive(Debug)]
enum IPAddress {
    V4(String),
    V6(String)
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String
}

fn main() {
    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    go_to(&IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1")
    });

    go_to(&IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1")
    });

    println!("{:?}", IPAddress::V4(String::from("127.0.0.1")));
    println!("{:?}", IPAddress::V6(String::from("::1")));

    let x = 5;
    let y: Option<u8> = Some(45);

    let sum: Option<u8> = match y {
        Some(i) => Some(i + x),
        None => None
    };
    println!("{:?}", sum);
}

fn route(ip: IpAddressKind) {
    println!("{:?}", ip);
}

fn go_to(ip: &IpAddress) {
    println!("{:#?}", ip);
}