/*
----- MUTABLE VARIABLES -----
fn main() {
    let mut x = 5;
    println!("variable x is {x}");
    x = 6;
    println!("variable x is {x}");
}
*/

/*
----- SHADOWING -----
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
*/

/*
----- TUPLES -----
*/
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is {y}");

    let five_hundred = tup.0;

    println!("I will print 500: {five_hundred}");
}
