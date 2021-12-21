use core::cmp::Ordering;
fn main() {
    // let number_list = vec![34, 50, 25, 100, 64];
    //
    // let result = largest(&number_list);
    // println!("The largest is {}", result);
    // // println!("The vec is {:?}", number_list);
    // let number_list = vec![102, 34, 6000, 798, 25, 8, 2];
    // let result = largest(&number_list);
    // println!("The largest is {}", result);
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest is {}", result);


    // let integer = Point { x: 5, y: 10 };
    // println!("x = {}", integer.x());
    // let float = Point { x: 3.0, y: 4.0 };
    // println!("{:#?}", integer);
    // println!("{:#?}", float);
    // let int_and_float = Point { x: 3.0, y: 4 };
    // println!("{:#?}", int_and_float);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p1 = Point{x:3.0,y:3.0};
    println!("p1  = {}", p1.distance_from_origin());

}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &mut item in list.iter_mut() {
//         // if item.cmp() > largest {
//         //     largest = item;
//         // }
//
//         match item.cmp(&largest) {
//             Ordering::Greater => largest = item,
//             Ordering::Less => continue,
//             Ordering::Equal => continue,
//         }
//     }
//
//     largest
// }

/// 在结构体中何用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
/// 只为f32类型提供的方法 其他类型无法使用
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// 在枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}
