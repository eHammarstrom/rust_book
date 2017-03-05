#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn enlarge(&mut self, factor: u32) {
        self.length = factor * self.length;
        self.width = factor * self.width;
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.length > other_rect.length && self.width > other_rect.width
    }
}

fn main() {
    let mut rect = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area());

    rect.enlarge(10);

    println!("{}", rect.area());

    println!("rect is: {:#?}", rect);

    let mut rect_mini = Rectangle { length: 10, width: 10 };

    println!("rect_mini is: {:#?}", rect_mini);

    println!("can rect hold rect_mini? {}", rect.can_hold(&rect_mini));

    rect_mini.enlarge(100);

    println!("rect_mini is: {:#?}", rect_mini);

    println!("can rect hold rect_mini? {}", rect.can_hold(&rect_mini));

    let square = Rectangle::square(10);

    println!("square is: {:#?}", square);
}
