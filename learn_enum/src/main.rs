#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Receive(String),
    Send(String),
    Connect(IpAddr),
    // Info { ipv4: IpAddr, ipv6: IpAddr }
}

impl Message {
    fn return_self(&self) -> &Message {
        self
    }
}

fn main() {
    let ip_addr4 = IpAddr::V4(127, 0, 0, 1);
    let ip_addr6 = IpAddr::V6(String::from("::1"));

    let connection = Message::Connect(ip_addr4);
    println!("connection: {:#?}", connection);

    println!("self: {:#?}", connection.return_self());
    // println!("ipv4: {:#?}", ip_addr4);
    // println!("ipv6: {:#?}", ip_addr6);
}
