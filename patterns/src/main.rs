/*
 * ----- Patterns on let statements -----
fn main() {
    let (x, y, z) = (1, 2, 3);
}
 */

/*
 * ----- Patterns on if statements -----
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
 */

/*
 * ----- Patterns in while loops -----
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
}

 */

/*
 * ----- Implicit patterns in for loops -----
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}
 */

/*
 * ----- Handling refutable patterns -----
fn main() {
    let Some(x) = Some(5) else {
        return;
    };
}
 */

/*
 * ----- Conditional blocks shadowing variables -----
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // This hits, as new y is made
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}
 */

/*
 * ----- Match range with ..= -----
fn main() {
    let x = 5;

    match x {
        1..5 => println!("Number in range 1 to 4"),
        1..=5 => println!("Number in range 1 to 5"),
        _ => println!("Number outside of range"),
    }
}

 */

/*
 * ----- Destructuring to break apart values -----
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point {x, y: 0} => println!("On the x-axis at {x}"),
        Point {x: 0, y} => println!("On the y-axis at {y}"),
        Point {x, y} => {
            println!("On neither axis: {x}, {y}");
        },
    }
}
 */

/*
 * ----- Remaining parts with .. -----
fn main() {
    struct Point {
        x: i32,
        _y: i32,
        _z: i32,
    }

    let origin = Point { x: 0, _y: 0, _z: 0};

    match origin {
        Point {x, ..} => println!("x is {x}"),
    };
}
 */

/*
 * ----- Complex matches with match guards (if statement) -----
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
}
 */

/*
 * ----- @ Bindings -----
 */
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 11 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
