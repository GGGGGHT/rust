// rust collection
// vector
// string
// map
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

}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


