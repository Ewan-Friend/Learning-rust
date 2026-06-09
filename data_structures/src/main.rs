/*
 * ----- Updating a Vector -----

fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
 */

/*
 * ----- Reading Elements ----- 

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
}
 */ 

/*
 * ----- Iterating Over Vector Values ----- 

fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        print!("{i} -> ");
        *i += 50;
        println!("{i}");
    }
}
*/

/*
 * ----- Appending with push ----- 

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    push_str(s1, s2);
    println!("s2 is {s2}");
}
 */

/*
 * ----- Appending with + ----- 

fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s  = s1 + "-" + &s2 + "-" + &s3;
    // or 
    // let s = format!("{s1}-{s2}-{s3}");
    
    println!("{s}")
}
 */

/*
 * ----- Iterating over strings -----
 */

fn main() {
    let string = "Здравствуйте";

    for c in string.chars() {
        println!("{c}")
    }

    for b in string.bytes() {
        println!("{b}")
    }
}
