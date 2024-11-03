// trait 定义是一种将方法签名组合起来的方法
#![allow(dead_code)]

use std::fmt::{self, format, Display};
// 导入trait
use learn_trait::Summary;
// 导入结构体
use learn_trait::Tweet;
use learn_trait::NewsArticle;

#[derive(Debug,PartialEq)]
enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

// 重写Display trait，指定对应类型的println!输出格式
impl Display for FileState {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     match *self {
         FileState::Open => write!(f, "OPEN"),
         FileState::Closed => write!(f, "CLOSED"),
     }
   }
}

impl Display for File {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<{} ({})>",
             self.name, self.state)
   }
}

impl File {
  fn new(name: &str) -> File {
    File {
        name: String::from(name),
        data: Vec::new(),
        state: FileState::Closed,
    }
  }
}

trait Draw {
  fn draw(&self) -> String;
}

impl Draw for u8 {
  fn draw(&self) -> String {
    format!("u8: {}", *self)
  }
}

// x为存储堆上的动态类型，该类型实现了trait约束
fn draw1(x: Box<dyn Draw>) -> String {
  x.draw()
}

trait Iterator {
  type Item; // 关联类型
  fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
  value: u32,
}

impl Counter {
  fn new(initial_value: u32) -> Counter {
    Counter {
      value: initial_value
    }
  }
}

impl Iterator for Counter {
  type Item = u32; // 定义具体类型

  fn next(&mut self) -> Option<Self::Item> {
    let value = self.value;
    self.value += 1;
    Some(value)
  }
}

// 定义默认泛型类型，如果没有传递类型，默认和当前数据类型一致
trait Add<RHS=Self> {
  type Output;

  fn add(self, rhs: RHS) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// 不同类型使用Add约束，则需要手动传递类型
impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Self::Output {
    Millimeters(self.0 + (other.0 * 1000))
  }
}

// 为trait添加特征约束
trait OutlinePrint: Display {
  fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
  }
}

impl OutlinePrint for File {}

// 由于 Display 是外部trait 以及 Vec 是外部类型，因此不能直接重写trait签名方法
// impl Display for Vec<String> {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "[{}]", self.join(", "))
//   }
// }
// 可以通过元组类型包裹 (newtype)
struct Wrapper(Vec<String>);

impl Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn get_option_value<T: Display>(value: Option<T>) {
  if let Some(x) = value {
    println!("Got a value: {}", x);
  } else {
    println!("Got None");
  }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Rust books"),
        content: String::from("how to write rust code"),
        author: String::from("hu yang"),
        location: String::from("China"),
    };

    println!("1 new tweet: {}", tweet.summarize());

    println!("read an article: {}", article.read()); // 重写实现
    println!("read an tweet: {}", tweet.read()); // 默认实现

    println!("detail an article: {}", article.detail()); // 默认实现
    println!("detail an tweet: {}", tweet.detail()); // 重写实现

    // 给类型实现trait后，可以直接通过类型调用trait声明的方法
    println!("Tweet call newString: {:?}", Tweet::newString());
    println!("NewsArticle call newString: {:?}", NewsArticle::newString());
    // 直接调用trait声明的方法，需要指定类型
    println!("Summary call newString: {:?}", <NewsArticle as Summary>::newString());
    // 等同于article.detail()
    println!("NewsArticle call detail: {:?}", NewsArticle::detail(&article));
    
    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6);
    f6.outline_print();

    let u8_num = 123u8;
    let u8_res = u8_num.draw();
    println!("draw1 res: {:?}", draw1(Box::new(u8_num)));
    println!("u8 draw res: {:?}", u8_res);

    let mut counter = Counter::new(11);
    let cn1 = counter.next();
    let cn2 = counter.next();
    get_option_value(cn1);
    get_option_value(cn2);
    get_option_value(Some(counter.value));

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
