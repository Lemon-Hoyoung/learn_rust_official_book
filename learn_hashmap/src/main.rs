// HashMap不在预导入库中
// HashMap数据存储在heap上
// HashMap是同构的：所有的K和所有的V必须是同一种类型
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("Rust".to_string(), 100);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 使用zip方法构建两个vec迭代器的元组迭代器
    // ["Blue", "Yellow"] [10, 50] => ("Blue", 10), ("Yellow", 50)
    let tuple_iter = teams.iter().zip(initial_scores.iter());
    // for (team, score) in tuple_iter {
    //     println!("{}, {}", team, score);
    // }
    
    // 使用collect将元组迭代器整合为其他数据，需要指定整合的数据类型，这里定义为HashMap类型
    let scores: HashMap<_, _> = tuple_iter.collect(); // 
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // get方法返回枚举值
    match score {
        Some(s) => println!("team_name score: {:?}", s),
        None => println!("team not exist"),
    }

    let mut values = HashMap::new();
    values.insert("Rust".to_string(), 100);
    values.insert("Rust".to_string(), 90);
    println!("values initial: {:?}", &values); // {"Rust": 90}

    // 直接insert会覆盖之前的数据，使用entry(key).or_insert(value)则是先判断
    // 如果没有就插入，有就不插入, entry返回key是否存在的枚举值，or_insert返回最终value的可变引用
    let js_entry_enum = values.entry(String::from("Javascript"));
    println!("js_entry_enum: {:?}", js_entry_enum);
    let js_insert_value = js_entry_enum.or_insert(60);
    println!("js_insert_value: {:?}", js_insert_value);

    let rust_entry_enum = values.entry(String::from("Rust"));
    println!("rust_entry_enum: {:?}", rust_entry_enum);
    let rust_insert_value = rust_entry_enum.or_insert(80);
    println!("rust_insert_value: {:?}", rust_insert_value);

    println!("values last: {:?}", &values); 


    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("text_map: {:?}", text_map);

    
}
