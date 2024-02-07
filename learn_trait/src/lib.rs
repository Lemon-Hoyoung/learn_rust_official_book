use std::fmt::Display;
use std::fmt::Debug;
// trait是struct类型方法的扩展
// 使用trait的步骤：
// 1. 定义trait，声明描述行为方法签名
pub trait Summary {
  fn summarize(&self) -> String; // 此时只有函数签名，没有具体实现
  fn read(&self) -> String { // 如果添加具体实现，则该实现为默认实现
    format!("This is a summary: {}", self.summarize()) // 默认实现可以调用其他方法
  }
  fn detail(&self) -> String {
    format!("This summary detail is {}", self.read())
  }
}
// 2. 为类型实现trait
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
  fn read(&self) -> String {
    format!("I have Read this article from {}", self.headline) // 已有默认实现，此处为重写实现
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}, {}", self.username, self.content)
  }
  fn detail(&self) -> String {
    format!("The detail of the tweet is {}", self.read())
  }
}

// item为实现了Summary这个trait的类型，在本例子中为 NewsArticle or Tweet
pub fn notify_impl_trait(item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// impl trait 是 trait Bound 写法的语法糖，以下为 trait Bound 写法：
pub fn notify_trait_bound<T: Summary>(item: T) {
  println!("Breaking news! {}", item.summarize());
}

// 支持变量trait类型相加，意味着变量需要同时实现多个不同trait：
pub fn notify_impl_trait_plus(item: impl Summary + Display) {
  println!("Breaking news! {}", item.summarize());
}
// or
pub fn notify_trait_bound_plus<T: Summary + Display>(item: T) {
  println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T: Display + Clone, U: Display + Debug>(t: T, u: U) -> String {
  format!("{}, {}", t, u)
}
// 可以使用where语法整理
pub fn some_function_where<T, U>(t: T, u: U) -> String
  where T: Display + Clone,
    U: Display + Debug
{
  format!("{}, {}", t, u)
}

// 可以指定返回类型为某种trait，一个trait的实现类型可能有多个（Summary实现类型有NewsArticle和Tweet）
// 但是Rust要求返回的类型必须是同一种类型
pub fn create(content: &str) -> impl Summary {
  NewsArticle {
    headline: String::from("This is an article"),
    location: String::from("wuhan"),
    author: String::from("Hu Yang"),
    content: String::from(content),
  }
}

// 1. trait约束：
// 当trait或者想要实现trait的类型位于crate的本地作用域，才能为该类型实现trait，
// 比如可以为本地作用域的类型Tweet实现Display trait，
// 也可以为Vec<T>类型实现本地trait Summary
// 为了保证代码的一致性，不允许为外部类型实现外部trait，比如不能为Vec<T>实现Display trait

// 2. 调用实现trait的类型实例上的方法时候优先级：
// 实现trait类型上的重写(具体)实现 > trait定义上的默认实现 

// 3. trait定义 的方法是 实现trait的类型 的方法的 超集

// 4. 约束函数输入类型为多个trait相加，相加的trait越多，约束越多

// 5. 赋值操作时，被赋值的变量必须实现了Copy trait
// 赋值指的是直接转移值，创建新指针指向这个值不算