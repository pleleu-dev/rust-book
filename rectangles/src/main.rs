#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Point(i32, i32);

impl Point {
    fn x(&self) -> i32 {
        self.0
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect: Rectangle = Rectangle::new(20, 30);

    dbg!("{:#?}", &rect);
    dbg!(rect.area());
    dbg!(rect.can_hold(&Rectangle::new(10, 20)));

    let p = Point(1, 2);
    println!("{}", p.x());
}
