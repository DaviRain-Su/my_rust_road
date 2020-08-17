extern crate restaurant;

#[test]
fn test_restaurant() {
    restaurant::eat_at_restaurant();
    restaurant::serving::serve_order();
    restaurant::serving::take_order();
    restaurant::serving::take_payment();
}