pub mod anamil;
#[cfg(test)]
mod tests {
    use crate::anamil::cat;
    use crate::anamil::dog;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn use_cat() {
        cat::hello();
        assert_eq!(true, cat::is_cat());
    }
    #[test] 
    fn use_dog() {
        dog::hello();
        assert_eq!(true, dog::is_dog());
    }
}
