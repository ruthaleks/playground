#[derive(Debug)]
pub struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size:u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

fn main() {
    let w = 30; 
    let h = 50;

    println!("The area of the rectangle is: {}", func::area(w, h));

    // refactoring with tuples
    let rect = (w, h);
    println!("The area of the rectangle is: {}", tup::area(rect));

    // refactoring with structs
    let rectangle = Rectangle{ width:w, height:h};
    println!("The area of the rectangle is: {}", stru::area(&rectangle));

    println!("rectangle is {:?}", rectangle);
    println!("rectangle is {:#?}", rectangle);

    // struct methods 
    println!("The area of the struct is: {}", rectangle.area());

    let rect1 = Rectangle{ width: 30, height: 50};
    let rect2 = Rectangle{ width: 10, height: 40};
    let rect3 = Rectangle{ width: 60, height: 45};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated functions 
    let sq = Rectangle::square(h);
    println!("A square is {:#?}", sq);
    
}

mod func{
    pub fn area(width: u32, height:u32 ) -> u32 {
        width*height
    }
}

mod tup{ 
    pub fn area(dimentions: (u32, u32)) -> u32 {
        dimentions.0 * dimentions.1
    }
}

mod stru{
    use crate::Rectangle;
    pub fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}