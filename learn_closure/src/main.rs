// 闭包：
// 1. 匿名函数
// 2. 可以保存为变量、作为参数
// 3. 可在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
// 4. 可从其定义的作用域捕获值
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// 通过根据闭包从所在环境中捕获值的方式不同，Rust推断出具体使用哪个Trait：
// 1. 取得所有权：FnOnce（所有闭包都实现了该Trait）
// 2. 可变借用：FnMut（没有移动捕获变量的实现了该Trait）
// 3. 不可变借用：Fn（无需可变访问捕获变量的实现了该Trait）

fn main() {
    generate_workout(24, 2);

    // 闭包可以访问定义在它所在作用域内的变量，普通函数则不能
    // 因此闭包会产生额外的内存开销
    let x = 123;
    let y = vec![1, 2, 3];
    let compare_x = |y: i32| y == x;
    println!("compare_x_true: {}", compare_x(123));
    println!("compare_x_false: {}", compare_x(321));

    // move关键字强制转移环境中的变量(y)的所有权
    let compare_y = move |z: Vec<i32>| z == y;
    // println!("y: {:?}", y);
    println!("compare_y_true: {}", compare_y(vec![1, 2, 3]));
}

// 该函数睡了2秒后返回输入的值，模拟复杂运算
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // 定义声明闭包
    // 1. 闭包不要求标注参数和返回值的类型，因为闭包通常短小，只在狭小的上下文中工作，编译器通常
    // 能够自动推断类型（也可以手动标注类型），并且最终只会推断出唯一的参数/返回值类型
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity + 1));
        println!("Next, do {} pushups again!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

// 让struct持有闭包：
// struct定义需要知道所有字段的类型，因此需要指明闭包的类型
// 每个闭包实例都有自己唯一的匿名类型，即使两个闭包签名完全一样
// 所以需要使用：泛型和Trait Bound

// Fn Trait（由标准库提供）
// 所有闭包至少实现了以下Trait之一：
// Fn  FnMut  FnOnce


// 下面的例子中，约束结构体中的calculation为一个闭包类型，而且输入参数和返回参数均为u32类型
// 使用calculation闭包进行计算，value缓存计算的值：
struct Cacher<T>
where
    T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}