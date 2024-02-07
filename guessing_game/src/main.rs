use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Start Guessing Game!");
    println!("Please input guessing game number:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("神秘数字{}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Your Guess Number: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("input value is not a number, please input again!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            },
        }
    }
}
