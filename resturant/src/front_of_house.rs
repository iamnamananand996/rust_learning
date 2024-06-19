mod hosting;
mod server;

pub struct Breakfast {
    pub toast: String,
    seasonal_food: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_food: String::from("mango"),
        }
    }
}
