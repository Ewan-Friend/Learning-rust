/*
----- IF STATEMENTS -----

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
*/

/*
----- CONDITIONAL LET -----
fn main() {
    let condition = true;
    let number = if condition {5} else {4};

    println!("number is {number}")
}
*/

/*
----- INFINITE LOOP -----
fn main () {
    loop {
        println!{"again!"}
    }
}
*/

/*
----- COUNTER -----
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/

/*
----- LOOP LABEL -----
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 3 {
                break;
            }
            if count == 4 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/

/*
----- LOOPING THROUGH ARRAY -----
*/
fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a {
        println!("element is {element}");
    }
}

