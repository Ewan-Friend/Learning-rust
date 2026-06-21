pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----- Successful test ----- 
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // ----- Failing test -----
//    #[test]
//    fn it_fails() {
//        panic!("Make this test fail");
//    }
    // ----- Using Result Syntax ----- 
    #[test]
    fn _it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4{
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }   

    // ----- Can test private functions -----
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
