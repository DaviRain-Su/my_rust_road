use std::println;

use regex::Regex;
const TO_SEARCH: &str = "On 2017-12-31, happy. On 2018-01-01, New Year.";

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH) {
        println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str()
        );
    }
    // println!("Hello, world!");

    let re = Regex::new(
        r"(?x) 
            (?P<year>\d{4}) # the year
            -
            (?P<month>\d{2}) # the month
            - 
            (?P<day>\d{2}) # the day
        ",
    )
    .unwrap();

    let caps = re.captures("2018-01-01").unwrap();
    // for val in caps.iter() {
    // println!("{:?}", val);
    // }
    println!("years = {}", &caps["year"]);
    println!("month = {}", &caps["month"]);
    println!("day = {}", &caps["day"]);

    let date = String::from("2018-01-01");
    let after = re.replace_all(&date, "$month/$day/$year");
    println!("after = {}", after);
    println!("date = {}", date);
}
