// trait 定义是一种将方法签名组合起来的方法

// 导入trait
use learn_trait::Summary;
// 导入结构体
use learn_trait::Tweet;
use learn_trait::NewsArticle;

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
}
