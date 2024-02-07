use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    // let args: Vec<String> = env::args().collect();
    
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     // 使用process::exit(code);强制退出程序
    //     // code = 1 表示异常退出, code = 0 表示
    //     process::exit(0);
    // });
    
    // 使用迭代器的方式优化上述代码：
    let args = env::args();
    
    let config = Config::new_by_iterator(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // 使用process::exit(code);强制退出程序
        // code = 1 表示异常退出, code = 0 表示
        process::exit(0);
    });

    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);
    //     process::exit(0);
    // }
    run(config).unwrap_or_else(|error| {
        eprintln!("Application error: {}", error);
        process::exit(0);
    })
}

// 在上面例子中如果所有的地方都用println!，则所有信息（错误信息和正确信息）都会输出到标准输出
// 此时若调用 cargo run > out.txt （> out.txt表示将标准输出内容保存到out.txt文件中）
// 无论是错误信息还是正确信息都将被输出到out.txt文件

// 因此可以在错误场景下用eprintln!替换println!，将错误信息输出到标准错误
// 此时再调用 cargo run > out.txt
// 错误信息会打印在控制台，正确信息输出到out.txt