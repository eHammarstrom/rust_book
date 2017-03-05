fn main() {
    let (x, y) = (1, 2); // variable bind x and y by pattern

    let x64: i64 = 10;
    
    /* ERROR: re-assignment of immutable variable
    let c: i8 = 64; // 'char'
    c = 10;
    */

    /* Passing
    let mut c: i8 = 64;
    c = 65;
    */

    let c: i8 = 64; // 'char'

    let t: i32;
    t = 32;

    let ans: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let gold: u8 = 0xFF;

    println!("{}, {}, {}, {}, {}, {}, {}",
             x, y, x64, c, t, ans, gold);
}
