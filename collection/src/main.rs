/// 动态数组vector
/// 字符串 string
/// 哈希表 hashmap

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

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}