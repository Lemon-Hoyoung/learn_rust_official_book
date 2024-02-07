// 同一层级可以直接使用
// pub开放子模块给父模块使用

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {
      println!("Adding to waitlist");
      super::guesting::book_table();
    }
  }

  mod guesting {
    pub fn book_table() {
      println!("Booking table");
    }
  }
}

mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    _seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        _seasonal_fruit: String::from("Apple"),
      }
    }
  }
}

use crate::front_of_house::hosting;
use std::io::{self, Write};

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("wheat");
  println!("I'd like {} toast please", meal.toast);
  // meal._seasonal_fruit = String::from("blueberries");
}