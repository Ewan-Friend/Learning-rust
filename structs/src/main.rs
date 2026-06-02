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

/*
 * ----- RECT AREA PROGRAM ----- 
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Not a Method - but a constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(11);

    println!("{sq:#?} \nis a square!!!");
    println!("rect1 is {rect1:#?}");
    // OR dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
