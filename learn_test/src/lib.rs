// 测试函数需要使用test属性(attribute)标注
// Attribute就是一段Rust代码元数据
// 函数上方添加#[test] 即可把函数变成测试函数

// 使用 cargo new [libraryName] --lib 可以创建一个library
// 使用 cargo test 命令会构建Test Runner可执行文件，运行标注了test的函数并报告是否成功

// 测试失败的场景：
// 1. 测试函数触发panic
// 2. 每个测试运行在一个新线程，当主线程看见某个测试线程挂掉了，那个测试标记为失败了

// 一些测试宏：assert!  assert_eq!  assert_ne!

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // 将外部模块所有内容导入
    use super::*;

    // 1. assert!：判断值为bool类型，如果为true，则测试成功，为false则调用panic!，测试失败
    // 自定义信息可以作为第二个参数传入，会被传递给format!
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(smaller.can_hold(&larger), "Smaller can't hold larger");
    }

    // 2. assert_eq! 和 assert_ne! 测试相等性
    // assert_eq! 判断两个参数是否相等，相等则测试成功，不等则自动打印两个参数的值，panic并测试失败
    // assert_ne! 和 assert_eq! 刚好相反
    // 由于断言失败会使用debug格式打印参数，因此要求参数实现了PartialEq和Debug Traits
    // 自定义信息作为第三个参数传入，会被传给format!
    #[test]
    fn assert_eq_failure() {
        let result = add(2, 3);
        assert_eq!(result, 4);
    }

    #[test]
    fn assert_ne_failure() {
        let result = add(2, 2);
        assert_ne!(result, 4);
    }

    // panic触发测试失败
    #[test]
    fn failure() {
        panic!("Test Failed");
    }

    // should_panic验证某些函数是否按照期望发生panic恐慌，发生恐慌则测试成功，未恐慌则测试失败
    // 如果添加expected参数，则额外检查panic恐慌的文本中是否包含expected文本，包含则测试成功，否则测试失败
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 使用Result返回来测试，Ok则测试成功，Err则测试失败
    #[test]
    fn result_ok() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {
            value
        }
    }
}

// cargo test [testName] 可以指定运行单个测试：
// cargo test result_ok

// cargo test [part of testName] 可以指定运行多个测试：
// cargo test assert ===> 可以运行assert_eq_failure和assert_ne_failure两个测试

// cargo test --[test properties] --[binary file properties]
// cargo test 命令后面接两个--参数，第一个传递给cargo test，第二个传递给生成的二进制文件
// 比如控制线程数量：cargo test -- --test-threads=1

// 在测试函数上打赏ignore标记：#[ignore]，直接运行cargo test，ignore标记的函数将被忽略
// 想要运行被忽略的测试：cargo test -- --ignored




// 单元测试和集成测试：
// 单元测试：对单个模块进行隔离测试，可以测试private接口
// 在tests模块上标注 #[cfg(test)]，只有运行cargo test才编译运行

// 集成测试：在库外部，只能使用public接口，可以测试多个模块
// 集成测试在不同目录，无需#[cfg(test)]标注