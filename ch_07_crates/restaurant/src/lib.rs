mod front_of_house { // not public but can be accessed by eat_at_restaurant as sibling
    pub mod hosting { // child module items are private to parent by default
        pub fn add_to_waitlist() {} // need both module and function to be public

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path - preferred
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order(){}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super keyword to go up one level relatively
    }

    fn cook_order() {}
}