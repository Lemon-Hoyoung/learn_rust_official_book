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

    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// if let 用于模式匹配和解构，通常用于处理Option、Result等枚举类型，允许同时模式匹配和取值：
// let some_value = Some(42);

// if let Some(x) = some_value {
//     println!("Got a value: {}", x);
// } else {
//     println!("Got None");
// }

// if let Some(x) = v 中，let Some(x) = v 部分为匹配体，如果匹配成功，则返回true
// if 判断为true则执行 {} 中的语句

// 模式匹配的方法：
// 1. match
// 2. let
// 3. if let（本质是用let匹配）
// 4. while let（同上）
// 5. for循环
// 6. 函数入参
// let for match需要全覆盖匹配（不能够遗漏场景）
// if let 则允许只匹配一种场景