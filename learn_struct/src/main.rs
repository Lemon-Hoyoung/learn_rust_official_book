#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl创建对应结构体的方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let square_geometry = Rectangle::square(31);
    let geometry = Rectangle {
        width: 30,
        height: 50,
    };
    let compare1 = Rectangle {
        width: 10,
        height: 45,
    };
    let compare2 = Rectangle {
        width: 33,
        height: 28,
    };

    println!("geometry: {}", geometry.area());
    println!("square: {}", square_geometry.area());
    println!("can hold compare1: {}", geometry.can_hold(&compare1));
    println!("can hold compare2: {}", geometry.can_hold(&compare2));
    println!("can hold square_geometry: {}", geometry.can_hold(&square_geometry));

    println!("{:#?}", geometry);
}