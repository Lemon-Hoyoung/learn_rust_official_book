// 1. 可变可以借用给不可变引用，不可变不能借用给可变引用
// 2. 同一作用域，最多只能有一个可变引用
// 3. 作用域从定义开始到使用结束，可变能同时借用给可变引用和不可变引用，前提是两个引用作用域无交集

// 从读写来看待引用：
// 可变引用：对某块内存区域 （写 & 读） 的入口
// 不可变引用：对某块内存区域 （只读） 的入口
// 1. 可写的入口 可以缩小为 只读的入口，只读的入口 不能够放大为 可写的入口（熵减性）
// 2. 写锁：同时只能由一个入口来 写（唯一性）
// 3. 读写锁：从一个入口 写 的时候，不能够从其他入口 读（统一性）

fn main() {
    let mut s = String::from("hello");
    let s2 = test(&mut s);
    let s3 = &s;
    let s4 = &s;

    println!("{}", s3);

    let s5 = &mut s;
    s5.push_str("s5 push");
    println!("{}", s5);
    // println!("{}, {}", s3, s4);
    // println!("{}, {}", s3, s);

    /// 为防止数据竞争，被借用的作用域内，原变量不能修改
    let mut i = 1;
    let i1 = &mut i;
    // i = 2; // error
    *i1 = 2;
    println!("i1: {}", i1); // 如果通过解引用修改i1的值，则不能在此处读i，因为统一性

    let num = 100_u8.checked_add(200); // u8溢出，checked检查算法，运算返回Option类型
    if let Some(s) = num {
        println!("checked num: {}", s);
    } else {
        println!("None");
    }

    let num = 500_u16.wrapping_mul(500); // u16溢出，wrapping回绕算法，值为 250000 % (2 ^ 16)
    println!("wrapping num: {}", num); // 53392

    let num = 32760_i16.saturating_add(10); // i16溢出，saturating饱和算法，值为最接近的最大值或最小值
    println!("saturating num: {}", num); // 32767

    let (num, is_overflowed) = 255_u8.overflowing_add(2); // u8溢出，overflowing溢出算法，值为元组(result, overflowed)
    println!("overflowing num: {}, {}", num, is_overflowed); // 1, true
}

fn test(s1: &mut String) -> &mut String {
    s1.push_str("haha");
    // println!("{}", s1);
    s1
}


#[test]
fn test_number() {
    let x = 42.0_f32;
    let y = 42.0f32;
    assert_eq!(x, y)
}

#[test]
fn test_plus() {
    let twenty = 20;
    let twenty_one: u32 = 21;
    let twenty_two = 22u32;

    let addition = twenty + twenty_one + twenty_two; // 相加的时候自动推导twenty类型
    assert_eq!(addition, 63)
}

#[test]
fn test_move() {
    let mut a = 2_i32;
    let b = 3_i32;
    a <<= b;
    assert_eq!(a, 16)
}