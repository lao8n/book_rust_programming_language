mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // internal code is one struct but api another

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // doesn't need to reference front_of_house
}