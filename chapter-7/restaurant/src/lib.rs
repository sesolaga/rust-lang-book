use crate::outside_module::inner_of_outside::some_outside_function;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            
        }

        fn seat_at_table() {

        }
    }

    mod serving {
        fn take_order() {

        }

        fn serve_order() {

        }

        fn take_payment() {

        }
    }
}

mod outside_module;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    some_outside_function();
}