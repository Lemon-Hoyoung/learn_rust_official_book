// enum Option<T> {
//    Some(T),
//    None,
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<i32>)
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(value) => {
            match value {
                None => -1,
                Some(v) => v,
            }
        }
    }
}

fn main() {
    let none_value = value_in_cents(Coin::Quarter(None));
    let quarter_value = value_in_cents(Coin::Quarter(Some(123)));
    let penny_value = value_in_cents(Coin::Penny);

    println!("none_value: {}", none_value);
    println!("quarter_value: {}", quarter_value);
    println!("penny_value: {}", penny_value);
}
