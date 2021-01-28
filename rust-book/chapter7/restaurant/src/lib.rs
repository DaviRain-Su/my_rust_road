
mod front_of_house;

mod back_of_house;

// 使用关键字将路径导入作用域中
// 使用绝对路径导入
// use crate::front_of_house::hosting;
// 使用相对路径导入
use self::front_of_house::hosting;
pub use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    front_of_house::hosting::add_to_waitlist();


    //---------------------------------------------

    //选择黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");

    //修改我们想要的面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 接下来的这一行无法通过编译，我们不能看到货更换随着食物附带的季节性水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;


    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

