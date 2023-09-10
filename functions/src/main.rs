fn main() {
    println!("Hello, world!");
    print_labeled_measurement(66, 'h');

    let x = {
        let y = 4;
        y + 3
    };

    println!("{x}");

    let z = {
        let a = func_with_return();
        plus_one(a)
    };

    println!("{z}");

    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("the measurement of x is: {}{}", x, unit_label);
}

fn func_with_return() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn f(x: i32) -> i32 {
    x + 1
}
