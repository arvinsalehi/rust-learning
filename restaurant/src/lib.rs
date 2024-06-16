mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn payment() {}
        fn deliver_feedback() {}
    }

    mod security {
        fn escort() {}
        fn patrol() {}
        fn eject() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub mod menu {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn combo1() -> Breakfast {
            Breakfast {
                toast: String::from("crispy"),
                seasonal_fruit: String::from("lime"),
            }
        }
    }
}

pub fn eat_at_resturant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let meal = menu::Breakfast::combo1();
    println!("combo1 has {} toast", meal.toast);
}
