// 模块
#[warn(dead_code)]
mod factory {
    pub mod product_refigerator {
        pub fn produce_refigerator() {
            println!("produce refigerator!");
        }
    }
    pub mod product_washing_machine {
       
        pub fn produce_washing_machine() {
            println!("produce washing maching!");
        }
    }

}

fn main() {
    factory::product_refigerator::produce_refigerator();
    println!("Hello, world!");
}
