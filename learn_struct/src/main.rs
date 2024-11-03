#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl创建对应结构体的方法
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width: width, height: height }
    }

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

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
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

    let user1 = User::new(String::from("281910751@qq.com"),
        String::from("lennonhu"));

    println!("user1: {:#?}", user1);
    let user2 = User {
        username: String::from("LennonHY"), // 不会被user1.username覆盖
        ..user1 // user1.email发生了所有权转移
    };
    // 无法访问user1.email 但是其他的可以
    println!("user1.username: {:#?}", user1.username);
    println!("user2: {:#?}", user2);
}