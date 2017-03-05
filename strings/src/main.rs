fn main() {
    /* slow flood
    let mut s: String = String::new();

    loop {
        s.push('x');
    }
    */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{} ... {}", s3, s2);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = t1 + "-" + &t2 + "-" + &t3;

    println!("{}", t);

    let t1 = String::from("tic");

    let t = format!("{}-{}-{}", t1, t2, t3);

    println!("{}", t);

    let len = String::from("Hola").len(); // 4 bytes

    let len = String::from("Здравствуйте").len(); // 24 bytes (unicode 2 byte per cyrillic char)
}
