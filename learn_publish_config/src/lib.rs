//! # LearnPublish Crate
//! 使用markdown写文档，使用cargo doc --open生成HTML文档并打开,
//! 使用cargo test能够测试编写的文档示例

/// add two values
/// # Example
/// ```
/// let number_one = 123;
/// let number_two = 321;
/// let result = learn_publish_config::add(number_one, number_two);
/// 
/// assert_eq!(result, 444);
/// ```
pub fn add(value: u32, another: u32) -> u32 {
  value + another
}

/// determine a number whether an odd number
/// # Panics
/// ```
/// let number: i32 = -123;
/// 
/// assert_ne!(learn_publish_config::is_odd(number), true);
/// ```
pub fn is_odd(value: i32) -> bool {
  if value < 0 {
      panic!("value must be positive");
  }
  if value % 2 == 1 {
      println!("it is an odd number");
      true
  } else {
      println!("it is an even number");
      false
  }
}

/// transform Option value to Result value
/// # Errors
/// ```
/// let option_value = None;
/// 
/// assert_ne!(learn_publish_config::get_value_from_options(option_value), Ok(1));
/// ```
pub fn get_value_from_options<T>(options: Option<T>) -> Result<T, &'static str> {
  if let Some(value) = options {
      Ok(value)
  } else {
      Err("Enum value is None!")
  }
}