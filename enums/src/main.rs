#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // The rest...
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
       match self {
           UsState::Alabama => year >= 1819,
           UsState::Alaska => year >= 1959,
           _ => {println!("Not a registered state!"); false}
       } 
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState) 
}

/* 
 * ----- Coin Matching ----- 
 *

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
 */

/*
 * ----- If Let Control Flow ----- 
 *
 *fn main(){
 *    let config_max = Some(3u8);
 *    match config_max {
 *      Some(max) => println!("maximum is configured to be {max}"),
 *       _ => (),
 *   }
 *}
 * Is equal to:
 *
 fn main(){
     let config_max = Some(3u8);
     if let Some(max) = config_max {
         println!("The maximum is configured to be {max}");
     }
 }
 */

/*
 * ----- Let Else Control Flow ----- 
 */

fn main(){
    describe_state_quarter(Coin::Quarter(UsState::Alabama));
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
   let Coin::Quarter(state) = coin else {
       return None;
   };

   if state.existed_in(1900){
       Some(format!("{state:?} is pretty old, for America!"))
   } else {
       Some(format!("{state:?} is relatively new."))
   }
}
