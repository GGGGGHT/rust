struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn other(&self) {
        println!("call other, width: {}, height: {}", self.width, self.height);
    }
}
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn main(){
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30,50);
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect1)
        rect1.area()
    );
    println!("rect1 is {:#?}", rect1);

    rect1.other();
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据，就像我们在“变量与数据交互的方式
    // （一）：移动”部分讲到的一样。在这个例子中，总体上说我们在创建 user2 后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型，所以我们在“变量与数据交互的方式（二）：克隆” 部分讨论的行为同样适用。
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User1.active: {}", user1.active);
    println!("User1.email: {}", user1.email);
    // println!("User1.username: {}", user1.username);
    //                                ^^^^^^^^^^^^^^ value borrowed here after move

    let subject = AlwaysEqual;
    let subject = AlwaysEqual{};
}

