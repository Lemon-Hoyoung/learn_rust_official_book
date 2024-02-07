// learn_package_path 是 Cargo.toml 中的 name 字段的值
use learn_package_path::eat_at_restaurant;
use rand::Rng;

mod category; // 声明引用外部模块
use category::info::out_fruits;

fn main() {
    eat_at_restaurant();
    out_fruits();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number: {}", secret_number);
}
