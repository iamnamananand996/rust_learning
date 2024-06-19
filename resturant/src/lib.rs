mod front_of_house;
pub use crate::front_of_house::Breakfast;
use rand::{thread_rng, CryptoRng};

fn item_3() {}

fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();

    let meal = front_of_house::Breakfast::summer("Bread");

    // println!("{:?}", meal)
}
