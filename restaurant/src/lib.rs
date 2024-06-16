mod front_of_house;
pub use crate::front_of_house::hosting;



// mod back_of_house;

pub mod menu;
pub fn eat_at_resturant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    let meal = menu::Breakfast::combo1();
    println!("combo1 has {} toast", meal.toast);
}
