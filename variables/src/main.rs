/*
 * Copyright (c) 2021. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */
/// Rust 内存会自动地在拥有它的变量离开作用域之后进行释放 会调用一个叫drop的特殊函数
// const MAX_POINTS: u32 = 100_000;
fn main() {
    // let tup = (500, 6.4, 1);
    // let condition = false;
    //
    // let f = if condition {
    //     5
    // } else {
    //     "true"
    // };
    // // let (x,y,z) = tup;
    // println!("The value of y is: {}", tup.0);
    // println!("The value of y is: {}", tup.1);
    // println!("The value of y is: {}", tup.2);
    //
    // let a = [1, 2, 3];
    // println!("{}", a[0]);
    // println!("{}", a[1]);
    // println!("{}", a[2]);
    //
    // println!("result : {}", five());
    // println!("plus one: {}", plus_one(3));
    // let a = [1, 2, 3, 4, 5];
    // let mut idx = 0;
    // while idx < 5 {
    //     println!("a[{}] = {}", idx, a[idx]);
    //     idx += 1;
    // }

    // for e in a.iter() {
    //     println!("the value is: {}", e);
    // }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }

    // let mut s = String::from("hello");
    // s.push_str(", world!");
    //
    //
    // let s1 = String::from("hello");
    // // let s2 = s1;
    //
    // // rust 不会自动地创建数据的尝试拷贝. 任何自动的赋值 操作都可以视为高效的
    // // error
    // println!("{}, world!", s1);
    //
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");

    takes_ownership(s);

    // println!("s = {} ", s);

    let x = 5;

    makes_copy(x);
    println!("x = {}", x);

    let (str, len) = calculate_length("haha".to_string());

    println!("str = {}, len = {}", str, len);
}
/// 将一个值赋值给另一个变量时就会转移所有权.当一个持有堆数据的变量离开作用域时,它的数据就会被drop清理回收.
fn makes_copy(x: u32) {}

fn takes_ownership(s:String) {}

// 使用元组返回多个值
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s,length)
}
// fn five() -> i32 {
//     // 5
//     return 5;
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }