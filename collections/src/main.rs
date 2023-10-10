fn main() {
    let mut v: Vec<i32> = Vec::new();

    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 6];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    if let Some(number) = v2.get(3) {
        println!("The fourth element is {number}");
    } else {
        println!("There is no fourth element");
    }

    let fifth: Option<&i32> = v2.get(4);
    match fifth {
        Some(number) => println!("The fifth element is {number}"),
        None => println!("There is no fifth element"),
    }

    for number in &mut v2 {
        *number = *number * 2
    }


    let mut v1: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v1 {
      v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v1[0];
    println!("{a} {b}");

    let row = vec!{
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(4.12),
        SpreadsheetCell::Text(String::from("Hello"))
    };

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
