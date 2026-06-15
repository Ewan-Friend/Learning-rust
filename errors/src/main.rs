/*
 * ----- Triggering a panic -----

fn main() {
    panic!("crash and burn")
}
 */

/*
 * ----- Triggering a panic II -----

fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
 */

/*
 * ----- Handling based on return value -----

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening he file: {error:?}")
            }
        }
    };
}

//----- Alternative ----- 
//use std::fs::File;
//use std::io::ErrorKind;
//
//fn main() {
//    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//        if error.kind() == ErrorKind::NotFound {
//            File::create("hello.txt").unwrap_or_else(|error| {
//                panic!("Problem creating the file: {error:?}");
//            })
//        } else {
//            panic!("Problem opening the file: {error:?}");
//        }
//    });
//}
 */

/*
 * ----- Unwrap ------
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
 */

/*
 * ----- Expect -----
use std::fs::File;

fn main() {
    let greeting_file= File::open("hello.txt")
        .expect("hello.txt should be included in this project!");
}
 */

/*
 * ----- Using ? operator for propegation -----

fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Or just:
    // fs::read_to_string("hello.txt")
}

// ? also works with Option<T> values 
// Exits early on None 
// Continutes on Some 
 */

/*
 * ----- Return type on main() -----
 */

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
