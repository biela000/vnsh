struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

fn main() {
    let user1 = build_user("someone@example.com", "someuser123");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.email); // Trying to use user1.username would result in an error because
                                 // the value has been moved

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
    }
}
