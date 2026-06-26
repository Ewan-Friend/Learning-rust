/*
 * ----- Using closures -----

fn main() {
//    fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // Function
//    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closures -----
//    let add_one_v3 = |x|             { x + 1 };
//    let add_one_v4 = |x|               x + 1  ;
}
 */

/*
 * ----- Caputuring references / moving ownership -----
 */

/*
 * ----- Immutable references -----
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("Afer calling closure: {list:?}");
}
 */

/*
 * ----- Mutable references -----
fn main(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
 */

/*
 * ----- Forcing closure ownership -----

use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
 */

/*
 * ----- FnMut closure call example -----
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

fn main() {
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    // Closure call here
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
 */

/*
 * ----- Defining iterators -----
 */
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

/*
 * ----- Calling next -----
 */

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

/*
 * ----- Consuming iterators with sum ------
 */
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

/*
 * ----- Adapting and collecting iterators using map ----- 
 */
#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

