// 所有的迭代器都实现了 Iterator trait
// Iterator trait 定义于标准库，定义如下：
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// type Item 和 Self::Item 定义了与此该trait关联的类型
// Item类型用于next方法返回类型（迭代器返回类型）
// Iterator trait 仅要求实现一个方法：next
// next每次返回迭代器中的一项，返回结果包裹在Some中，迭代结束返回None，可直接在迭代器上调用next

use std::fmt::Display;

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let mut v1_iter = v1.iter();
    
    // 隐式调用了 v1.into_iter() 因此发生了所有权转移！
    // for v in v1 {
    //     println!("v: {}", v);
    // }
    // 所有权发生转移，打印失败
    // println!("v1: {:?}", v1);

    // for in 会取得v1_iter所有权，v1所有权仍然存在
    // for v in v1_iter {
    //     println!("{}", v);
    // }
    // println!("v1: {:?}", v1); // 不会报错

    // 每次调用 next 都会改变原来迭代器的值（消耗型适配器）
    // 但是不会取得其中数据的所有权！
    match v1_iter.next() {
        Some(&value) => println!("next_value: {}", value),
        None => println!("end of the iteration"),
    }
    println!("v1_iter: {:?}", v1_iter); // Iter([2, 3, 4])

    // Iter上的sum求和方法也是一种消耗型适配器
    // sum求和：取得迭代器所有权然后反复调用next遍历元素，然后将当前元素放入总和，结束后返回

    // let total: i32 = v1_iter.sum();
    // println!("sum of elements of v1_iter: {}", total);

    // turbofish 语法，和上面写法功能等同:
    println!("sum of elements of v1_iter: {}", v1_iter.sum::<i32>());
    // println!("v1_iter after sum: {:?}", v1_iter); // error：已被转移所有权

    // map（迭代器适配器），传入一个闭包，对前一个迭代器中每个元素进行映射，返回一个迭代器
    // 迭代器是惰性的，必须最终调用消耗型适配器，否则迭代器什么都不做
    // collect是一个消耗型适配器，将结果收集到集合Vector中
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    // filter（迭代器适配器），传入一个闭包，保留返回true的元素作为新迭代器的元素
    // filter会对迭代器元素进行引用，所以使用into_iter转移所有权，否则需要多次解引用
    let v3: Vec<_> = v1.into_iter().filter(|y| *y < 3).collect();
    println!("v3: {:?}", v3);

    let mut self_iter = Counter::new();
    print_value_from_option(self_iter.next());
    print_value_from_option(self_iter.next());
    print_value_from_option(self_iter.next());
    print_value_from_option(self_iter.next());
    print_value_from_option(self_iter.next());
    print_value_from_option(self_iter.next());
    print_value_from_option(self_iter.next());

    println!("other_iterator_value: {}", using_other_iterator_trait_methods());
}

// 几个迭代方法：
// iter：在不可变引用上创建迭代器
// into_iter：创建的迭代器会获得所有权
// iter_mut：迭代可变引用

// 创建自定义迭代器：实现next方法
// 实现一个遍历到5的迭代器：
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn print_value_from_option<T>(option: Option<T>)
    where T: Display
{
    if let Some(value) = option {
        println!("value: {}", value);
    } else {
        println!("it's end");
    }
}

fn using_other_iterator_trait_methods() -> u32 {
    let sum: u32 = Counter::new()
        .zip(Counter::new()
        .skip(3))
        .map(|(a, b)| a * b)
        .sum();
    sum
} // (1 * 4) + (2 * 5)