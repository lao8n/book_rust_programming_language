struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
    email: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = User { // moves the data - cannot use user1 as a whole anymore
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{} {}", user1.email, user2.email);
}
