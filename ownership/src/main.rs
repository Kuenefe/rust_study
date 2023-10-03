fn main() {

    let mut user1: User = User {
        email: String::from("text@test.de"),
        username: String::from("blubb"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.active);

    let mut user2: User = build_user(String::from("hallo@h.de"), String::from("kun"));

    println!("{}", user2.email);

    let mut user3: User = User {
        ..user1
    };


    println!("{}", user3.username);
    //println!("{}", user1.username);

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 2,
    }
}
