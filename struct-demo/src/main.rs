#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {

    fn square(size: i32) -> Rectangle {
        Rectangle {width: size, height: size}
    }

    fn get_area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}



fn main() {
    let rect = Rectangle {
        width: 100,
        height: 10
    };

    let rect2 = Rectangle {
        width: 120,
        height: 10,
    };

    let rect3 = Rectangle::square(100);

    let result = rect.get_area();
    let holdable = rect.can_hold(&rect2);

    println!("{}", result);
    println!("{:#?}", rect);
    println!("rect can hold rect2: {}", holdable);
    println!("Hello, world!");
}
