/*
 * ----- Generic function decleration (without valid traits) -----

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
 */

/*
 * ----- Structs using Generics (mix & match) -----

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x 
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10};
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("i_and_f.x = {}", integer_and_float.x());
}
 */

/*
 * ----- Method definitions using Generics -----

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let q = Point { x: 5.5, y: 2.1 };

    println!("distance from origin of q: {:.16}", q.distance_from_origin());
}
 */

/*
 * ----- Defining and implementing traits -----

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// If from a different library (as usual):
// use aggregator::{SocialPost, NewsArticle, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

}

use std::fmt::Display;
// ----- Using Traits as parameters ----- 

pub fn notify(item: &(impl Summary + Display)){
    println!("Breaking news! {}", item.summarize());
}

// ----- Trait bound alternate syntax -----

pub fn _notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// ----- Trait bound where clauses ----- 
use std::fmt::Debug;

fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{ unimplemented!()}
 */

 /*
  * ----- Lifetime annotations -----

 fn main(){
    // let reference: &i32;
    // let reference_with_lifetime: &'a i32;
    // let mutable_reference_with_lifetime: &'a mut i32;
    
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // result has the same as the
                                                                  // "smaller scope"
        println!("The longest string is \"{result}\"");
    }
 }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
  */

 /*
  * ----- Lifetime in struct definitions -----

 // ImportantExcerpt can't outlive reference in part field
 struct ImportantExcerpt<'a> {
     part: &'a str,
 }

impl<'a> ImportantExcerpt<'a> {
    // Return type gets the lifetime of &self
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i.part);
}
  */

 /*
  * ----- Static lifetime -----

 fn main() {
    // s lives for the entire duration of the program
    let s: &'static str = "I have a static lifetime";
 }
  */

 /*
  * ----- All in one! -----
  */

 use std::fmt::Display;

 fn longest_with_an_announcement<'a, T>(
     x: &'a str,
     y: &'a str,
     ann: T
     ) -> &'a str
 where 
     T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {result}");}
