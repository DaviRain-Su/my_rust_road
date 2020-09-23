#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub mod factory;

// 重导出
pub use crate::factory::product_refigerator::produce_refigerator;
pub use crate::factory::product_washing_machine::produce_washing_machine;
