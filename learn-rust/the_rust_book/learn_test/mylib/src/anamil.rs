
pub mod dog {
    pub  fn hello() {
        println!("🐯🐯");
    }
    pub fn is_dog() -> bool {
        true
    }
}

pub mod cat {
    pub fn hello () {
        println!("🐱🐱");
    }
    
    #[test]
    fn test_hello(){
        hello();
    }
    pub fn is_cat() -> bool{
        true
    }
}