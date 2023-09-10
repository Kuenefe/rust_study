fn main() {
    let animals: (&str, &str, &str) = ("cow", "dog", "cat");

    let (cow, dog, cat) = animals;

    println!("{}", animals.1);
    println!("");


    let shopping_list: [&str; 3] = ["salad", "cheese", "mushrooms"];
    let second_array = ["hello"; 3];

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("error");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("error");
            panic!("Invalid input!");
        }
    };

    if index < shopping_list.len() {
        let element: &str = shopping_list[index];
        println!("From the index you have chosen: {}", element);
    } else {
        println!("not in bounds");
    }

    let t = ([1;2], [3; 4]);
    let (a, b) = t;

    println!("first number: {}", a[0]);
    println!("second number: {}", t.1[0]);
    println!("sum: {}", a[0] + t.1[0]);
}
