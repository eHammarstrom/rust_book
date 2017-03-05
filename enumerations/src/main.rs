enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn to_string(&self) -> String {
        match *self {
            IpAddr::V4(w, x, y, z) => format!("{}.{}.{}.{}", w, x, y, z),
            IpAddr::V6(ref s) => s.clone(),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{}", home.to_string());
    println!("{}", loopback.to_string());

    let num1: u32 = 5;
    let num2: Option<u32> = Some(5);

    match num2 {
        Some(n) => println!("{}", n + num1),
        None => println!("num2 was none"),
    }

    let absent_number: Option<i32> = None;

    match absent_number {
        Some(n) => println!("{}", n),
        None => println!("num2 was none"),
    }
}
