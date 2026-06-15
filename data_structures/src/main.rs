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

fn main() {
    let string = "Здравствуйте";

    for c in string.chars() {
        println!("{c}")
    }

    for b in string.bytes() {
        println!("{b}")
    }
}
 */

/*
 * ----- Creating hash maps -----

fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {score}\n");

    // prints in arbitrary order
    for (key, value) in &scores {
        println!("{key}: {value}")
    }
}

*/

/*
 * ----- Updating hashmap -----

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}
 */

/*
 * ----- Adding key + value only if key isnt present -----

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
 */

/*
 * ----- updating value based on old value -----
 */

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
