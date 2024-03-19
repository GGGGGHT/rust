// rust collection
// vector
// string
// map

use std::collections::HashMap;


fn main() {

    let v = vec![1, 2, 3, 4, 5];

    // let _does_not_exist = &v[100];
    let _does_not_exist = v.get(100);
    match _does_not_exist {
        Some(value) => {
            println!("value: {}", value);
        },
        None => {
            println!("None");
        }
    }

    let mut v = vec![100,2,57];

    for i in &mut v {
        *i += 50;
        println!("{i}")
    }

    dbg!(&v);

    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];

    dbg!(&row);

    let data = "initial contents";
    let s = data.to_string();

    let mut s = "hello".to_string();
    s.push_str(" world");

    println!("s: {}", s);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 使用 add获取所有权
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    println!("s3: {}, s2:{}", s3, s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 使用format宏不会获取所有权
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s1}, {s2}, {s3}, {s}");

    let hello = "Здравствуйте";

    let s = &hello[0..2];
    println!("{s}");

    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    scores.insert(blue.clone(), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:#?}", scores);
    // let team_name = String::from("Blue");
    let score = scores.get(&blue).copied().unwrap_or(0);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("filedName: {field_name}, filedValue: {field_value}");


    let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，
    // 如果不存在则将参数作为新值插入并返回新值的可变引用。这比编写自己的逻辑要简明的多，另外也与借用检查器结合得更好。
    let res = scores.entry(String::from("Yellow")).or_insert(50);
    println!("res: {res}");
    let res = scores.entry(String::from("Blue")).or_insert(50);
    println!("res: {res}");

    println!("{:?}", scores);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


