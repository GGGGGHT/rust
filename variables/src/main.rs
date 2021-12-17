/*
 * Copyright (c) 2021. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

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
    let a = [1, 2, 3, 4, 5];
    // let mut idx = 0;
    // while idx < 5 {
    //     println!("a[{}] = {}", idx, a[idx]);
    //     idx += 1;
    // }

    // for e in a.iter() {
    //     println!("the value is: {}", e);
    // }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn five() -> i32 {
    // 5
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}