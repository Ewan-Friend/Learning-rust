/* ----- PUSHING TO HEAP -----
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");
}
*/

/* ----- CLONING STRING -----
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();    

    println!("s1 = {s1}, s2 = {s2}");
}
*/

/* ----- OWNERSHIP IN FUNCTIONS ------
fn main() {
    let s = String::from("Hello");

    take_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn take_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}
*/

/* ----- RETURN & SCOPES -----
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String{
    let some_string = String::from("Yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/

/* ----- RETURN PARAMETER OWNERSHIP -----
fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}"); 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
 */

/* ----- USING REFERENCES INSTEAD -----
fn main() {
    let s = String::from("Hello");

    let len = calculate_length(&s);

    println!("Reference length of '{s}' is {len}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/

/* ----- MUTABLE REFERENCE -----
fn main() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("{s}")
}

fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}
*/

/* ----- SLICES -----
*/
fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("The first word is: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //SLICE
        }
    }

    &s[..] //Could just be '&s'
}
