fn main() {
    let mut s: String = String::new();

    let data: &str = "initial contents";

    let s: String = data.to_string();
    let s: String = "initial contents".to_string();

    let mut s1: String = String::from("foo");
    let s2: &str = "bar";

    s1.push_str("bar");

    println!("s2 is {s2}");

}
