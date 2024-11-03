// 字符串分两种类型 分别是标准库中的String 以及 核心语言 字符串切片（&str）
// 字符串切片分为 对String类型变量的引用(栈上指针指向heap) 和 字符串字面量(栈上指针指向只读内存)
// String本质上是Vector的包装, 因此可以使用很多Vector的方法
// String不支持索引取值，因为有的字符占用多个字节

// 字符char类型使用 Unicode 字符集，因此每个字符占用4字节
// 字符串String类型使用 UTF-8 编码，不同字符占字节数可能不同，因此不允许索引取值

// String是堆上str的指针，且拥有str的所有权
// &str也是堆上str的指针，但是没有所有权

fn main() {
    // 字符串字面量 => String
    let s1 = "Hello World";
    let mut s1_string = s1.to_string();

    // 根据字符串字面量创建String
    let s2 = String::from("Hello World");

    // String后面添加字符串切片
    let mut s3 = String::from("Hello ");
    s3.push_str("World"); // 在此处发生了隐式的可变借用

    // String后面添加一个字符
    let mut s4 = String::from("Hell");
    s4.push('o');

    // 两个String类型相加
    // 类似调用了fn add(self, s: &str) -> String {...}签名方法
    let s5 = String::from("Hello ");
    let s6 = String::from("World");
    let s7 = s5 + &s6; // s5会转移所有权

    // format!宏使用模板字符串连接多个字符串, 不会获取参数所有权
    let s8 = String::from("Hello");
    let s9 = String::from("World");
    let s10 = format!("{}, {}", s8, s9);

    s1_string.push_str(&s2);

    println!("s1: {}", s1);
    println!("s1_string: {}", s1_string);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    // println!("s5: {}", s5);
    println!("s6: {}", s6);
    println!("s7: {}", s7);
    println!("s8: {}", s8);
    println!("s9: {}", s9);
    println!("s10: {}", s10);

    let len = String::from("Hola").len(); // 1个字母对应1个字节
    let number_len = String::from("123").len(); // 1个数字对应1个字节
    let chinese_len = String::from("你好").len(); // 1个汉字对应3个字节
    let latin_len = String::from("Здравствуйте").len(); // 1个拉丁字母对应2个字节
    let sanskrit_len = String::from("नमस्ते").len(); // 梵文
    println!("len: {}", len); // 4
    println!("number_len: {}", number_len); // 3
    println!("chinese_len: {}", chinese_len); // 6
    println!("latin_len: {}", latin_len); // 24
    println!("sanskrit_len: {}", sanskrit_len); // 18
    let sanskrit = String::from("नमस्ते");
    for b in sanskrit.chars() {
        println!("{}", b); // 包含6个char 构成 4个字型簇
    }

    let s_test = String::from("Test");
    let te = &s_test[0..2];
    println!("sTest: {}", s_test);
    println!("te: {}", te);

}

// 由String类型构成的Vector切片：
// 1. 切片实现了Copy Trait，但无所有权
// 2. String数据未实现Copy Trait
// 如果直接将切片中的String数据赋值给变量，则涉及到了所有权转移，但切片没有所有权，因此会报错
// 比如以下代码报错：
// let test_string: String = String::from("hahaha");
// let test_vec: Vec<String> = vec![test_string];
// let move_test_string = test_vec[0];