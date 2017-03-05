fn main() {
    let num = 3;

    if num == 3 {
        println!("was 3");
    } else {
        println!("yeah no");
    }

    /* if requires bool unlike c, java etc.
    if num {
        println!("...")
    }
    */

    let is_less_than_10 = if num < 10 {
        true
    } else {
        false
    };

    println!("{}", is_less_than_10);

    let is_less_than_2 = if num < 2 {
        true
    } else {
        false
    };
    
    println!("{}", is_less_than_2);

    // educational purpose only, pls don't
    let is_great: Result<bool, String> = if num > 100 {
        Ok(true)
    } else {
        Err(String::from("not great enough"))
    };

    println!("{}", is_great.unwrap_or(false));
}
