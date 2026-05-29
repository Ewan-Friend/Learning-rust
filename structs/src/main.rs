/* ----- DEFINING STRUCT -----

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User{
        active: true,
        username: String::from("PingoWillSuffer"),
        email: String::from("pingo@frowntown.com"),
        sign_in_count:1,   
    };

    let user2 = User{
        email: String::from("pingo@happycity.com"),
        ..user1
    };

    let user3 = build_user(String::from("ewan.friend@email.co.uk"), String::from("ewan"));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
 */
