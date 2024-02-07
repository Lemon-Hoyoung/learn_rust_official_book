mod fruit_category {
  pub enum Fruit {
    Apple,
    Banana,
  }

  impl Fruit {
    pub fn is_apple(&self) -> bool {
      match self {
        Fruit::Apple => true,
        _ => false,
      }
    }
  }

  pub fn all_fruits() -> (Fruit, Fruit) {
    (Fruit::Apple, Fruit::Banana)
  }
}

pub fn out_fruits() {
  let (apple, banana) = fruit_category::all_fruits();
  if apple.is_apple() {
    println!("I like apples");
  }
  if banana.is_apple() {
    println!("I like bananas");
  }
}