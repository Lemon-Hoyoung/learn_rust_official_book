// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// Result上的 unwrap 方法相当于 match Result 的快捷方法：
// 如果Result值为Ok，返回Ok里面的值
// 如果Result值为Err，调用panic!宏

// Result上的 expect 和 unwrap 类似，但是expect可以额外指定panic!文本内容

use std::{fs::File, io::{ErrorKind, self, Read}};

fn main() {
    // 使用match来匹配错误：
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:?}", e),
    //         },
    //         other => panic!("Error opening the file: {:?}", other),
    //     },
    // };

    // 使用Result上的方法：
    // unwrap_or_else后面添加一个闭包
    // 如果Result值为Ok，返回Ok里面的值
    // 如果Result值为Err，调用内部闭包函数，将错误作为参数传递给闭包函数
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    println!("file: {:?}", f);

    let result = read_username_from_file_on_chain();
    println!("username: {:?}", result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 语句加? 表示下面语句的语法糖，如果成功继续执行，如果出错，立即返回，只能用于返回Result类型函数
    // 返回的错误类型会自动转换成要求的错误类型（前提是返回的错误上实现了转换成要求错误类型的from函数）
    let mut f = File::open("hello.txt")?;

    // let mut f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    // 同理，下面的写法和注释的写法是等价的
    f.read_to_string(&mut s)?;
    Ok(s)
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

// 可以改造上面的read_username_from_file为链式调用
fn read_username_from_file_on_chain() -> Result<String, io::Error> {
    let mut s: String = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 总结：
// 处理Result方法有: 
// 1. match手动处理
// 2. ?语法糖：函数内部流程控制（Ok场景不会返回，需要自己手动返回，
//    Err场景自动返回，所以需要关注所在函数返回类型）
// 3. unwrap：需要返回Ok且自动panic错误场景
// 4. expect：unwrap文本自定义
// 5. unwrap_of_else：unwrap错误流程自定义
// 6. if let Err(e)：只关注错误场景
// 7. is_err()：Ok场景返回false，Err场景返回true

// Err(E)是Result的错误枚举类型