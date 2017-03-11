fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&numbers));

    let chars = vec!['Y', 'M', 'C', 'A'];

    println!("The largest char is {}", largest(&chars));

    let int_point = Point { x: 2, y: 3 };
    let float_point = Point { x: 2.5, y: 3.5 };

    println!("int_point.x = {}", int_point.x());
    println!("float_point.x = {}", float_point.x());
}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
