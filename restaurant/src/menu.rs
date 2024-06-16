pub struct Breakfast {
    pub toast: String,
    pub seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(fruit: &str) -> Breakfast {
        Breakfast {
            toast: String::from("toast"),
            seasonal_fruit: String::from(fruit),
        }
    }

    pub fn combo1() -> Breakfast {
        Breakfast {
            toast: String::from("crispy"),
            seasonal_fruit: String::from("lime"),
        }
    }
}