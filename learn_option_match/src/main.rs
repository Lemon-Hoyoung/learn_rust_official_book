// enum Option<T> {
//    Some(T),
//    None,
// }

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<i32>)
}

struct Point {
    x: f32,
    y: f32,
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(value) => {
            match value {
                None => -1,
                Some(v) => v,
            }
        }
    }
}

// 匹配守卫（匹配时添加额外的条件判断）
fn temperature_match_guard(temperature: Option<i32>) -> () {
    match temperature {
        Some(x) if x <= 37 => println!("normal temperature"),
        Some(x) => println!("warning! you're having a fever! it's {} ℃", x),
        None => println!("unknown situation"),
    }
}

fn is_in_unit_square(point: Point) -> bool {
    match point {
        Point { x: x_pos @ -1.0..=1.0, y: y_pos @ -1.0..=1.0 } => {
            println!("x: {} and y: {}", x_pos, y_pos);
            true
        },
        _ => false
    }
}

fn main() {
    let none_value = value_in_cents(Coin::Quarter(None));
    let quarter_value = value_in_cents(Coin::Quarter(Some(123)));
    let penny_value = value_in_cents(Coin::Penny);

    println!("none_value: {}", none_value);
    println!("quarter_value: {}", quarter_value);
    println!("penny_value: {}", penny_value);

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let u = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("u: {:?}", u);
    println!("v: {:?}", v);

    temperature_match_guard(Some(36));
    temperature_match_guard(Some(38));
    temperature_match_guard(None);

    println!("point is in unit square: {}", is_in_unit_square(Point { x: -0.5, y: -1.0 }));

    println!("test_number_bigger_than_two: {}", test_match_some_bigger_than_two(2));
    println!("test_number_bigger_than_two: {}", test_match_some_bigger_than_two(3));
    println!("test_number_bigger_than_two: {}", test_match_some_bigger_than_two(1));
}

#[test]
fn test_matches() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
}

fn test_match_some_bigger_than_two(num: i8) -> bool {
    let number = Some(num);
    matches!(number, Some(value) if value > 2)
}

// 匹配的形式：
// 1. 匹配字面值
//（match x { 1 => println!("one"), _ => println!("any"), }）
// 2. 匹配命名变量
//（match x { Some(y) => println!("some value: {}", y), _ => println!("default"), }）
// 3. 或匹配
//（match x { 1 | 2 => println!("one or two"), _ => println!("any"), }）
// 4. 序列范围匹配
//（match x { 'a'..='z' => println!("lowercase letter"), _ => println!("other letter"), }）
// 5. 解构匹配（结构体、枚举、元组、数组）
// _能够忽略单个值的解构（不会转移所有权），..能够忽略多个值的解构