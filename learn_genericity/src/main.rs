// 泛型T写在函数、结构体，枚举、方法后的<>中
// 用作函数、结构体、枚举、方法内部的类型变量

// 泛型定义会在编译期间对实际使用的类型进行展开，以保证泛型定义的运行时性能（单态化）

// 函数泛型
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}

// 枚举泛型见 learn_option_match 和 learn_result章节笔记

// 结构体泛型以及方法泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// 看作 T = i32 的上述方法的实现
impl Point<i32> {
    fn get_x_i32(&self) -> &i32 {
        &self.x
    }
}

#[derive(Debug)]
struct OtherPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> OtherPoint<T, U> {
    fn mixup<V, W>(mut self, other: OtherPoint<V, W>) -> OtherPoint<T, W> {
        OtherPoint {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("result: {}", result);

    let char_list = vec!['h', 'u', 'y', 'a', 'n', 'g'];
    let char_result = largest(&char_list);
    println!("char_result: {}", char_result);

    let string_list = vec![String::from("Hello"), String::from("World")];
    for list_str in &string_list {
        println!("element in string_list: {}", list_str);
    }
    let string_result = &largest_clone(&string_list);
    println!("string_result: {}", string_result);



    let mut one_other_point = OtherPoint {
        x: 123,
        y: '1',
    };

    let two_other_point: OtherPoint<Vec<i32>, &str> = OtherPoint {
        x: vec![1, 2, 3, 4],
        y: "hello world",
    };

    let mix_other_point_result = one_other_point.mixup(two_other_point);

    println!("mix_other_point_result: {:?}", mix_other_point_result);
}
