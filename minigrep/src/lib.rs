use std::error::Error;
use std::{fs, env};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("The length of the arguments must be more than 3");
      }
      let query = args[1].clone();
      let filename = args[2].clone();

      // 环境参数中包含 CASE_INSENSITIVE 则Ok，is_err()后返回false
      // 不包含则Err，is_err()后返回true

      // Windows bash shell环境变量设置：
      // export CASE_INSENSITIVE=1 && cargo run "you smile" poem.txt
      // Windows cmd环境变量设置：
      // set CASE_INSENSITIVE=1 && cargo run "you smile" poem.txt
      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
  
      Ok(Config { query, filename, case_sensitive })
  }

  // 使用迭代器优化new构造函数
  pub fn new_by_iterator(mut args: std::env::Args) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("The length of the arguments must be more than 3");
    }
    args.next();
    
    let query = get_value_from_options(args.next())?;
    let filename = get_value_from_options(args.next())?;

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let result = if config.case_sensitive {
    search_by_iterator(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in result {
    println!("{}", line);
  }
  // println!("With text: \n{}", contents);
  Ok(())
}

// 如果查找的query中包含空格的两个单词可以加上双引号：
// cargo run "You smiled" poem.txt
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  // 迭代器，将每一行文本字符串切片传递给line
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

// 使用迭代器优化search函数
pub fn search_by_iterator<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();
  // 迭代器，将每一行文本字符串切片传递给line
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

fn get_value_from_options<T>(options: Option<T>) -> Result<T, &'static str> {
  if let Some(value) = options {
    Ok(value)
  } else {
    Err("value is None!")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
  }

  #[test]
  fn one_result_by_iterator() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search_by_iterator(query, contents))
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
  }
}