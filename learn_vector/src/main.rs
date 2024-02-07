// #[derive(Debug)]
fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v.push(1);

    let third: &i32 = &v2[2];
    println!("third: {}", third);

    match v2.get(100) {
        Some(third) => println!("third: {}", third),
        None => println!("no third"),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    v3.push(6);
    let first = &v3[0];
    println!("The first element is: {}", first);

    let mut v4 = vec![1, 2, 3, 4, 5];
    for i in &mut v4 {
        *i += 10; // 手动解引用取值运算
    }
    for i in &v4 { // 使用 &v4 和 v4.iter() 写法等价
        println!("{}", i); // 自动解引用打印
    }
    println!("上面的迭代未发生所有权Move，下面的发生了");
    for i in v4 {
        println!("{}", i);
    }

    // Vec<i32>是Vector类型，&[i32]是Vector切片类型引用
    let mut v5 = vec![38, 40, 28, 79, 27, 52, 76, 98, 34, 73];
    let v5_slice = &v5[0..5];

    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
        // for item in list {
        //     if *item > largest {
        //         largest = *item;
        //     }
        // }
        // 以引用的方式 (&item) 直接获取list中变量的值前提是list中的变量实现了Copy trait
        // 相当于已经知道了list迭代的值为引用类型，用item直接接收（赋值）迭代的值
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let result = largest(&v5);
    let result_slice = largest(&v5_slice);

    println!("maximun value in v5: {}", result);
    println!("maximun value in v5_slice: {}", result_slice);
}
