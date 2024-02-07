/// # 可变可以借用给不可变引用，不可变不能借用给可变引用
/// # 同一作用域，最多只能有一个可变引用
/// # 作用域从定义开始到使用结束，可变能同时借用给可变引用和不可变引用，前提是两个引用作用域无交集

fn main() {
    let mut s = String::from("hello");
    let s2 = test(&mut s);
    let s3 = &s;
    let s4 = &s;

    println!("{}", s3);

    let s5 = &mut s;
    println!("{}", s5);
    // println!("{}, {}", s3, s4);
    // println!("{}, {}", s3, s);

    /// 为防止数据竞争，被借用的作用域内，原变量不能修改
    let mut i = 1;
    let i1 = &mut i;
    // i = 2; // error
    println!("i1: {}", i1);
}

fn test(s1: &mut String) -> &mut String {
    s1.push_str("haha");
    // println!("{}", s1);
    s1
}
