/* 
 * ----- Coin Matching ----- 
 *

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // The rest...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState) 
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn main(){
    let dime_price = value_in_cents(Coin::Dime);
    println!("price of a dime is: {}", dime_price);

    let state_quarter = Coin::Quarter(UsState::California);
    let quarter_price = value_in_cents(state_quarter);
    println!("price of a quarter is {}", quarter_price);
}

*/

/*
 * ----- Option<T> match pattern ----- 

fn main(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
*/

/*
 * ----- Catch all & '_' -----
 */
fn main(){
    let dice_roll = 9;
    match dice_roll {
        3 => do_this_cool_thing(),
        7 => undo_cool_thing_and_do_bad_thing(),
        _ => (), //"we dont want to use a value"
    }

    fn do_this_cool_thing() {}
    fn undo_cool_thing_and_do_bad_thing() {}
    fn reroll() {}
}
