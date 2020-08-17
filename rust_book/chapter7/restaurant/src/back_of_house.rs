pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    //静态方法
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: toast.into(),
            seasonal_fruit: "peaches".into(),
        }
    }
}
pub enum Appetizer {
    Soup, 
    Salad,
}



fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::serve_order();
}

fn cook_order() {
    println!("cook_order");
}