/// 动态数组vector
/// 字符串 string
/// 哈希表 hashmap

/// OsString OsStr CString CStr 以str结尾为借用者版本 否则为所有者版本
use std::collections::HashMap;
fn main() {
    // let v: Vec<i32> = Vec::new();
    // 使用push向动态数组中添加元素
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v = {:#?}", v);

    let third = &v[2];
    println!("third = {}", third);
    // let non = &v[10];
    // println!("non = {}", non);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third elements."),
    }
    match v.get(10) {
        Some(ten) => println!("The third element is {}", ten),
        None => println!("There is no ten elements."),
    }

    /// 在存在指向动态数组元素的引用时尝试向动态数组中添加元素
    /// 动态数组中元素是连续存储的,插入新的元素后也许会没有足够多的空间将所有的元素依次相邻地放下
    /// 这就需要分配新的内存空间,并将旧的元素移动到新的空间上.第一个元素的引用可能会因为插入行为而指向被释放的内存.
    /*let mut ve = vec![1, 2, 3];
    let first = &ve[0];
    ve.push(4);
    println!("The first elements is: {}", first);*/
    let mut ve = vec![1, 2, 3];
    for ele in &mut ve {
        *ele += 50;
    }

    for ele in &ve {
        println!("{}", ele);
    }

    /// 在动态数组中使用定义的枚举来存储不同类型的值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(3.12),
    ];

    // let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    // 使用push_str 或 push向字符串中添加内容
    let mut foo = "foo".to_string();
    foo.push_str("str");
    println!("foo = {}", foo);

    let s1 = "Hello,".to_string();
    let s2 = "world!".to_string();

    /// 使用+将两个String拼接到一起 需要获取左侧字符串的所有权
    let s3 = s1 + &s2;
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");
    /// format宏不需要字符串的所有权
    let str4 = format!("{}-{}-{}", str1, str2, str3);
    let str5 = str1 + "-" + &str2 + "-" + &str3;

    // println!("str1 = {}", str1);
    println!("str2 = {}", str2);
    println!("str3 = {}", str3);
    println!("str4 = {}", str4);
    println!("str5 = {}", str5);

    let len = String::from("Hola").len();
    println!("len = {}", len);

    for c in "Hola".chars() {
        println!("c = {}", c);
    }

    let mut map = HashMap::new();
    map.insert("a", 10);
    map.insert("b", 1);

    println!("{:?}", map);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 30];
    let  scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    println!("{:#?}", scores);
    /// 对于实现了Copy trait的类型 如i32 它的值会被简单地复制到哈希映射中.而对于String这种持有所有权的值,其值将会转移且所有权会转移给哈希映射
    let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    let t = 10;
    let mut m: HashMap<String, i32> = HashMap::new();
    map.insert(&field_name[..], t);
    /// field_name和field_value从这一刻开始失效,如果使用它们则会导致编译出错
    // println!("k: {}, v: {}", field_name, field_value);
    println!("t = {}", t);
    /// 使用get来获取映射中的值
    let score = scores.get(&"Blue".to_string());
    println!("Blue score = {:#?}", score);

    for (k, v) in &scores {
        println!("{}:{}", k, v);
    }
    /// 覆盖旧值
    map.insert("Blue",30);
    for (k, v) in &mut map{
        println!("{}:{}", k, v);
    }

    /// 只在键没有对应值时插入数据
    let mut s = HashMap::new();
    s.insert(String::from("Blue"), 10);
    s.entry(String::from("Yellow")).or_insert(50);
    s.entry(String::from("Blue")).or_insert(50);
    for (k,v) in s {
        println!("k = {}, v = {}", k, v);
    }


    /// 使用旧值来更新值
    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}",text_map)
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
