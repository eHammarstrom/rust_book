enum Cell {
    Int(i32),
    Text(String)
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    let third: &i32 = &v[2];
    let third1: Option<&i32> = v.get(2);

    println!("{:?} , {:?}", third, third1);

    let mut multiV: Vec<Cell> = Vec::new();

    multiV.push(Cell::Int(42));
    multiV.push(Cell::Text(String::from("Hello, world!")));

    for cell in multiV {
        match cell {
            Cell::Int(x) => println!("{}", x),
            Cell::Text(x) => println!("{}", x),
        }
    }
}
