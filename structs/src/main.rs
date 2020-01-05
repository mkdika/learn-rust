fn main() {
    let user1 = User {
        username: String::from("maikel"),
        email: String::from("mkdika@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("username      : {}", user1.username);
    println!("email         : {}", user1.email);
    println!("sign in count : {}", user1.sign_in_count);
    println!("active        : {}", user1.active);
    println!("");

    let user2 = create_user(
        String::from("max@gmail.com"),
        String::from("maxmax")
        );
    println!("user2 is {:?}", user2);

    // creating instance from other instance
    // example from user1
    let user3 = User {
        username: String::from("jackson"),
        email: String::from("jackson@gmail.com"),
        ..user1
    };
    println!("user3 is {:?}", user3);
}

fn create_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
