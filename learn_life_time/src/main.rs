// 存储在stack上的变量赋值会Copy
// 存储在stack上的变量传递引用需要关注生命周期
// 存储在heap上的变量如果clone或者直接转移所有权则无需关注生命周期
// 存储在heap上的变量传递引用需要关注生命周期

// 生命周期标注 ('a): 描述多个引用生命周期间的关系，不会影响生命周期
// 单个生命周期标注无意义，意义在于多个生命周期标注的关系

// 一些生命周期标注的类型写法：
// &i32 普通引用
// &'a i32 带有显式生命周期的引用
// &'a mut i32 带有显式生命周期的可变引用

// 生命周期：
// 生命周期决定了内存中的数据何时被创建，何时被释放
// 隐式声明周期：
// 持有内存某块数据的所有权的变量，该变量生命周期为变量所在作用域（{}内）
// 也就是说 在所有权变量的作用域内，内存块数据不会被释放
// 显式生命周期标注：
// 显式生命周期标注的是 引用对应数据的所有权变量的作用域 的子集

fn main() {
    // life_time_introduce();
    let string1 = String::from("abcd");
    let longest_result;
    let result_return_str;
    let result_return_string;

    let import_value;
    {
         // 字符串字面量和字符串切片被硬编码到程序可执行文件中
         // 生命周期'static为整个程序运行的生命周期（静态生命周期）
        let string2 = "xyz";
         // 而String类型的变量生命周期只在当前作用域，离开当前作用域自动调用drop，回收内存
         // 如果写：
         // let string2 = String::from("xyz");
         // longest_result = longest(&string1, &string2);
         // 那就会报错，因为string2生命周期只在当前大括号中，而longest_result作用域在外部
    
        longest_result = longest(&string1, string2);

        result_return_str = return_str(&string1, string2);

        result_return_string = return_string(&string1, string2);

        import_value = ImportantExcerpt {
            part: string2
        };

        import_value.announce_and_return_part(string2);
        // 上述代码没问题，如果使用如下代码：
        // let string_value = String::from("xyz");
        // import_value = ImportantExcerpt {
        //    part: string_value.as_str()
        // }
        // 则会报错，因为 string_value 活得时间不够长
    }
    println!("longest result: {}", longest_result);
    println!("return_str: {}", result_return_str);
    println!("return_string: {}", result_return_string);

    println!("import_value: {:?}", import_value);

}

// fn life_time_introduce() {
//     let r; // r 生命周期开始
//     {
//         let x = 5; // x 生命周期开始
//         r = &x; // x 生命周期结束， x变量在此处被内存回收，却仍然被r借用
//     }
//     println!("r: {}", r); // r 生命周期结束， error：此处r仍然被使用
// }

// 函数泛型生命周期，函数名称后面<>中，约束函数参数和返回的生命周期不短于该生命周期
// 因此'a 生命周期为x y生命周期的交集
// x y 的生命周期标注是 输入生命周期 返回值的生命周期标注是 输出生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 返回引用类型需要标注生命周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 如果返回引用类型，且返回的值在函数内部作用域结束后被回收，就会发生悬垂引用
fn return_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 下面两行的写法会报错，发生悬垂引用：
    // let result = String::from("abc");
    // result.as_str()

    // 如果定义字符串字面量就不会报错
    let result = "abc";
    result
}
// 直接返回String这个值本身，那么发生了所有权转移，也不会报错
fn return_string(x: &str, y: &str) -> String {
    let result = String::from("xyz");
    result
}

// 结构体泛型生命周期，结构体名称后面<>中，约束结构体成员变量生命周期不短于该生命周期
// 成员变量 所引用的外部变量 生命周期必须存活时间比 结构体实例 生命周期时间长
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 结构体方法给impl后面和结构体名称后面分别添加 生命周期参数
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 结构体实例生命周期 不长于 外部引用变量生命周期
    // &self的生命周期即是 结构体实例的生命周期
    // 所以结构体方法中 引用 必须绑定struct字段引用的生命周期'a，引用 是独立的也可以
    fn announce_and_return_part(&self, announcement: &'a str) -> &str {
        println!("Attention please: {}", announcement);
        let announcement_bytes = announcement.as_bytes();

        for (i, &item) in announcement_bytes.iter().enumerate() {
            if item == b' ' {
                return announcement;
            }
        }
        self.part
    }
}


// 生命周期省略：
// 生命周期省略规则有三条，规则一应用于 输入生命周期，规则二、三用于 输出生命周期
// 编译器自动给fn函数和impl块应用三条规则，如果应用后仍然存在无法确定生命周期的引用，则报错
// 规则1： 每个引用类型的参数都有自己的生命周期
// 规则2： 如果只有1个输入生命周期参数，那么该生命周期被赋给所有输出生命周期参数
// 规则3： 如果有多个输入生命周期参数，但其中一个是&self或&mut self (方法)，那么self
// 的生命周期会被赋给所有的输出生命周期参数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// 因此上述函数first_word应用规则1后，函数签名：
// fn first_word<'a>(s: &'a str) -> &str
// 应用规则2后，函数签名：
// fn first_word<'a>(s: &'a str) -> &'a str
// 此时函数生命周期已能被编译器识别并通过

// 对于fn longest(x: &str, y: &str) -> &str这个函数签名，应用规则1后：
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
// 规则2不成立，因为有多个输入参数，规则3也不成立，因为没有&self参数
// 最后三条规则应用完，编译器仍然无法确定返回输出的生命周期，因此编译报错