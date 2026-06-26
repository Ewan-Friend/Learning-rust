// ----- Containing documentation comments (on root) ----- 
//! # Documentation Examples Crate
//!
//! 'documentation' is a display of example documentation that can be 
//! seen by running ```cargo doc --open```

// ----- Documentation comments -----
/// Adds one to the number given.
///
/// # Examples 
/// ```rust
/// let arg = 5;
/// let answer = documentation::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// ----- Re-exporting nested items ------ 

pub mod exportingColours{

    // now accessible by exportingColours::PrimaryColour etc...
    pub use self::kinds::PrimaryColour;
    pub use self::kinds::SecondaryColour;
    pub use self::utils::mix;

    pub mod kinds {
        /// The primary colours according to the RYB color model.
        pub enum PrimaryColour {
            Red,
            Yellow,
            Blue,
        }

        /// The secondary colors according to the RYB color model.
        pub enum SecondaryColour{
            Orange,
            Green,
            Purple,
        }
    }

    pub mod utils {
        use crate::exportingColours::kinds::*;

        /// Combines two primary colours in equal amounts to create 
        /// a secondary colour
        pub fn mix(c1: PrimaryColour, c2: PrimaryColour) -> SecondaryColour {
            unimplemented!()
        }
    }
}

