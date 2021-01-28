extern crate chapter3;
fn main() {

    chapter3::davirain::str_compose();
    
    // chapter3::davirain::compare_size();

    // let mut arr = [1, 2, 3, 4, 5];
    // println!("before arr: {:?}", arr);
    // chapter3::davirain::reset(&mut arr);
    
    // chapter3::davirain::zero_size();

    // chapter3::davirain::zero_size_loop();
    // chapter3::davirain::bottom_type();
    // chapter3::davirain::infer_demo();
    // chapter3::davirain::infer_generics();
    // chapter3::davirain::impl_method();
    // chapter3::davirain::associated_type();
    // chapter3::davirain::generics_trait();
    // chapter3::davirain::string_add();
    // chapter3::davirain::trait_inherit();
    // chapter3::davirain::trait_bound(); 
    // chapter3::davirain::trait_object();
    // chapter3::davirain::impl_trait();
    // chapter3::davirain::impl_clone();
    // chapter3::davirain::test_copy_trait();
    // chapter3::davirain::sync_send_trait();
    // chapter3::davirain::operate_def();
    // chapter3::davirain::auto_deref();
    // chapter3::davirain::manual_deref();
    // chapter3::davirain::fqsfd();
    // chapter3::davirain::from_into();
    // chapter3::davirain::trait_limit();
    // trait_object_dyn();
    // use_and_then();
    // use_map();
    // chapter3::davirain::trait_special();
    trait_object_dyn();
}


fn trait_object_dyn() {
    trait Shape {
        fn area(&self) -> f64;
    }
    trait Round {
        fn get_radius(&self) -> f64;
    }
    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }

    impl Round for Circle {
        // self : &Circle
        fn get_radius(&self) -> f64 {
            self.radius
        }
    }

    impl Shape for dyn Round { // impl trait to dyn trait 
        fn area(&self) -> f64 { // self : &Round
            std::f64::consts::PI * self.get_radius() * self.get_radius()
        }
    }

    let c = Circle { radius: 2f64 };
    println!("The Circle is {:?}", c);
    // c.area();
    let b = Box::new(Circle { radius: 4f64}) as Box<dyn Round>;
    let p = b.area();
    println!("The area is {}", p);
}

fn use_and_then() {
    fn sq(x: u32) -> Option<u32> {
        Some(x * x)
    }

    println!("{:?}", Some(2).and_then(sq).and_then(sq));
    let x = Some(2).and_then(sq).and_then(sq).unwrap();
    println!("the value is {}", x);
}

fn use_map() {
    let maybe_some_string = Some(String::from("hello, world"));
    let maybe_some_len = maybe_some_string.map(|s| s.len()).unwrap();
    println!("the maybe_some_string len is {}", maybe_some_len);
}