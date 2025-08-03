struct Rect {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

impl Rect {
    fn shape() -> String {
        return String::from("Rectangle");
    }
}

trait Shape {
    fn area(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn get_area(s: impl Shape) -> u32 {
    s.area()
}

fn main() {
    let r = Rect {
        width: 20,
        height: 30,
    };

    let s = Square { side: 4 };

    println!("{}", r.width);
    println!("{}", s.side);
    println!("{}", r.area());
    println!("{}", Rect::shape());
    println!("{}", get_area(r));
}

