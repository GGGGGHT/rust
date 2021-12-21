/*
 * Copyright (c) 2021. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

#[derive(Debug)]
struct Point<T,U> {
    x:T,
    y:U,
}
/// 在某些情况下可能会有一部分泛型参数声明于impl关键字后,而另一部分则声明于方法定义中
/// 泛型代码单态化.单态化是一个在编译期将泛型代码转换为特定代码的过程.它会将所有使用过的具体类型填入泛型参数从而得到有具体类型的代码.
impl <T,U> Point<T,U> {
    fn mixup<V,W>(self,other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y:'c' };
    let p3 = p1.mixup(p2);

    println!("{:#?}", p3);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}