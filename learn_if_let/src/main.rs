fn main() {
    let v = Some(2u8);
    if let Some(x) = v {
        println!("x: {}", x);
    } else {
        println!("others");
    }

    if Some(3) == v {
        println!("three again!");
    } else {
        println!("others again!");
    }
}

// if let 用于模式匹配和解构，通常用于处理Option、Result等枚举类型，允许同时模式匹配和取值：
// let some_value = Some(42);

// if let Some(x) = some_value {
//     println!("Got a value: {}", x);
// } else {
//     println!("Got None");
// }