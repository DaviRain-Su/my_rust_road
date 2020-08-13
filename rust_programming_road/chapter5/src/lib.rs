pub mod davirain {

    pub fn ownership_demo() {
        let orig = Box::new(5);
        println!("orig is {}", orig);
        // println!("orig address is {:?}", orig);
        let stolen = orig; // value moved here
                           // println!("{}", *orig); //  value borrowed here after move
        println!("stolen is {}", stolen);
    }

    pub fn value_semantic() {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }

    pub fn reference_semantic() {
        // use std::ops::Drop;
        #[derive(Debug, Clone)]
        struct A {
            a: i32,
            b: Box<i32>, // the trait 'Copy' cannot implement for this type
        }

        impl Drop for A {
            fn drop(&mut self) {
                println!("Drop {:?}", self);
            }
        }

        let a = A {
            a: 2,
            b: Box::new(4),
        };
        println!("a is {:#?}", a);
        {
            let b = a.clone();
            println!("b is {:#?}", b);
        }
        println!("a is {:#?}", a);
    }

    pub fn struct_demo_ownership() {
        #[derive(Debug, Copy, Clone)]
        struct A {
            a: i32,
            b: i32,
        }

        let a = A { a: 1, b: 2 };
        let b = a;
        println!("b is {:#?}", b);
        println!("a is {:#?}", a);
    }

    pub fn tuple_demo_ownership() {
        let a = ("a".to_string(), "b".to_string());
        let b = a;
        println!("b is {:#?}", b);
        let c = (1, 2, 3);
        let d = c;
        println!("d is {:#?}", d);
        println!("c is {:#?}", c);
    }

    pub fn immutable_var() {
        let x = "hello".to_string();
        println!("x = {}", x);
        // x += " world"; // cannot assign twice to immutable variable

        let mut x = "hello".to_string();
        println!("x = {}", x);
        x += " world";
        println!("x = {}", x);
    }

    pub fn word_scope_type() {
        let outer_val = 1;
        let _outer_sp = "hello".to_string();
        {
            let _inner_val = 2;
            // _outer_sp;
            // outer_val;
        }

        println!("outer_val = {}", outer_val);
        // println!("inner_val = {}", inner_val);
        // println!("outer_sp = {}", outer_sp);
    }

    pub fn match_scope_type() {
        let a = Some("hello".to_string());
        match a {
            Some(s) => println!("{:?}", s),
            _ => println!("nothing"),
        }
        // println!("{:?}", a); // used of partially moved value: 'a'
    }

    pub fn loop_scope_type() {
        let v = vec![1, 2, 3];
        for i in v {
            println!("{:?}", i);
            // println!("{:?}", v); // used of moved value: 'x'
        }
    }

    pub fn iflet_scope_type() {
        let a = Some(String::from("hello"));
        if let Some(s) = a {
            // a具有移动语义，进入if let会发生所有权的转移， if let创建了一块新的作用域
            println!("{:?}", s);
        }
    }

    pub fn whilelet_scope_type() {
        let mut optional = Some(0);
        while let Some(i) = optional {
            if 9 < i {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("i = {:?}, Try again!", i);
                optional = Some(i + 1);
            }
        }
    }
    pub fn function_scope_type() {
        fn foo(s: String) -> String {
            let w = String::from(", world");
            s + &w
        }
        let s = String::from("hello");
        let ss = foo(s); //  moved 's' into function scope
        println!("ss  = {:?}", ss);
        // println!("s = {}", s); // error use of moved value 's'
    }

    pub fn closure_scope_type() {
        let s = "hello".to_string();
        let join = |i: &str| s + i; // moved 's' into closure scope
        let ret = join(", world");
        println!("ret = {}", ret);
        // println!("s = {}", s); // error use of moved value 's'
    }
    pub fn ownership_borrowed_func_type() {
        fn foo(mut v: [i32; 3]) -> [i32; 3] {
            v[0] = 3;
            assert_eq!([3, 2, 3], v);
            println!("v in foo = {:?}", v);
            v
        }

        let v = [1, 2, 3];
        foo(v);
        println!("v = {:?}", v);
        assert_eq!([1, 2, 3], v);
    }

    pub fn ownership_borrowed_type() {
        fn foo(v: &mut [i32; 3]) {
            v[0] = 3;
            println!("v in foo = {:?}", v);
        }

        let mut v = [1, 2, 3];
        foo(&mut v);
        println!("v  = {:?}", v);
        assert_eq!([3, 2, 3], v);
    }

    pub fn bubble_sort_example() {
        fn bubble_sort(a: &mut Vec<i32>) {
            let mut n = a.len();
            while 0 < n {
                let (mut i, mut max_ptr) = (1, 0);
                while i < n {
                    if a[i] < a[i - 1] {
                        a.swap(i - 1, i);
                        max_ptr = i;
                    }
                    i += 1;
                }
                n = max_ptr;
            }
        }

        let mut a = vec![1, 3, 4, 2, 5, 6, 7, 8, 0];
        bubble_sort(&mut a);
        println!("a = {:?}", a);
    }

    pub fn borrow_check() {
        fn compute(input: &u32, output: &mut u32) {
            if 10 < *input {
                *output = 1;
            }
            if 5 < *input {
                *output *= 2;
            }
        }

        let i = 20;
        let mut o = 5;
        compute(&i, &mut o);
        println!("i = {}, o = {}", i, o);

        // let mut n = 20;
        // compute(&n, &mut n);
        //  cannot borrow 'n' as mutable because it it also borrowed as immutable
    }

    pub fn borrow_check_v2() {
        fn compute(input: &u32, output: &mut u32) {
            let cached_input = *input;
            if 10 < cached_input {
                *output = 2;
            } else if 5 < cached_input {
                *output *= 2;
            }
        }

        let i = 20;
        let mut o = 5;
        compute(&i, &mut o);
        println!("i = {}, o = {}", i, o);
    }

    pub fn deref_moved_type() {
        fn join(s: &String) -> String {
            let append = s; // cannot move let append = *s;
                            //解引用会导致移动语义类型获得所有权
            "hello".to_string() + append
        }

        let x = " world".to_string();
        let ret = join(&x); // 那么这里的话&x就会使野指针， &x指向无效的地址
        println!("ret = {}", ret);
    }

    pub fn life_time_checker() {
        // let r; // 'a
        // {
        //'b
        // let x = 5;
        // r = &x; // borrowed value does not live long enough
        // 借用的生命周期不能长于出借方的生命周期
        // } // end 'b
        // println!("r = {}", r);
    } // end 'a

    pub fn return_str_example() {
        // fn return_str<'a>() -> &'a str {
        fn return_string() -> String {
            let mut s = "Rust".to_string();
            for _ in 0..3 {
                s.push_str("Good ");
            }

            s
            // &s[..] // cannot return valye referencing local variable 's'
        }

        let x = return_string();
        println!("x  = {}", x);
    }

    pub fn foo_example() {
        // fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
        fn foo() -> String {
            let result = String::from("really long string");
            // result.as_str() // returns a value referencing data owned by the current function
            result
        }
        let _x = "hello";
        let _y = "rust";
        let ret = foo();
        println!("ret = {}", ret);
    }

    pub fn largest_str_example() {
        // fn the_longest(s1: &str, s2: &str) -> &str { 
        // 函数声明中的'a 可以看做一个生命周期参数，输入引用和输出引用都标记为‘a， 意味着输出引用（借用方）的生命周期
        //不长于输入引用（出借方）的生命周期
        fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
                                                // expected name lifetime parameter
            if s1.len() > s2.len() { s1 } else { s2 }
        }

        let s1 = String::from("Rust");
        let s1_r = &s1;
        {
            let s2 = String::from("C");
            let res = the_longest(s1_r, &s2);
            //当the_longest函数被实际调用时，借用检查器会根据函数签名时生命周期参数标记的具体情况进行检查
            println!("{} is the longest", res);
        }
    }

    pub fn the_longest_example_multi_life_time() {
        fn the_longest<'a: 'b , 'b> (s1: &'a str, s2: &'b str) -> &'b str {
            if s1.len() > s2.len() { s1 } else { s2 } 
        }
        let s1 = String::from("Rust");
        let s1_r = &s1;
        {
            let s2 = String::from("C");
            let res = the_longest(s1_r, &s2);
            //当the_longest函数被实际调用时，借用检查器会根据函数签名时生命周期参数标记的具体情况进行检查
            println!("{} is the longest", res);
        }
    }
    pub fn struct_life_time(){
        #[derive(Debug)]
        struct Foo<'a> {
            part: &'a str,
        }

        impl<'a> Foo<'a> {
            fn split_fist(s: &'a str) -> &'a str {
                s.split(',').next().expect("Could not find a ','")
            }
            fn new(s: &'a str) -> Self {
                Foo { part: Foo::split_fist(s) }
            }
            fn get_part(&self) -> &str {
                self.part
            }
        }

        let words = String::from("Sometimes think, the greatest sorrow than older");
        // let first = words.split(',').next().expect("Could not find a ','");
        // let f = Foo { part: first};
        let f = Foo::new(words.as_str());
        println!("f = {:?}", f);
        println!("f.part = {}", f.get_part());
    }

    pub fn static_str_example() {
        let x = "hello, world!";
        let y = x;
        println!("x = {}, y = {}", x, y);
    }

    pub fn find_first_word() {
         fn first_word(s: &str) -> &str {
             let bytes = s.as_bytes();
             for (i, &item) in bytes.iter().enumerate() {
                 if item == b' ' {
                     return &s[..i];
                 }
             }
             &s[..]
         }
         let res = first_word("hello rust");
         println!("res  = {}", res);

        //  fn print(s: &str){}
        //  fn print<'a>(s: &'a str) {}
        //  fn debug(lvl: usize, s: &str){}
        //  fn debug<'a>(lvl: usize, s: &'a str){}
        //  fn substr(s: &str, until: usize) -> &str{}
        //  fn substr<'a>(s: &'a str, until: usize) -> &'a str {""}
        //  fn get_str() -> &str {} //error 
        // fn frob(s: &str, t: str) -> &str {}
        // fn get_mut(&mut self) -> &mut T { "" }
        // fn get_mut<'a>(&'a mut self) -> &'a mut T {  } 
        // fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut command
        // fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut command {}
        // fn new(buf: &mut [u8]) -> BufWrite;
        // fn new<'a>(buf: &'a mut [u8]) -> BufWrite<'a>;
    }

    pub fn life_time_setting () {
        use std::fmt::Debug;
        #[derive(Debug)]
        struct Ref<'a, T: 'a>(&'a T);

        fn print<T>(t: T)
            where T: Debug, 
        {
            println!("print : t is {:?}", t);
        }
        
        fn print_ref<'a, T> (t: &'a T) 
        where 
            T: Debug + 'a,
        {
            println!("print_ref : t is {:?}", t);
        }
        let x = 6;
        let ref_x = Ref(&x);
        print_ref(&ref_x);
        print(ref_x);
    }
    pub fn trait_object_life_time() {
        trait Foo {}
        #[derive(Debug)]
        struct Bar<'a> {
            x: &'a i32,
        }
        impl<'a> Foo for Bar<'a> {}

        let num = 5;
        let box_bar = Box::new(Bar { x: &num });
        println!("box_bar = {:?}", box_bar);
        let _obj = box_bar as Box<dyn Foo>;
        // println!("obj is {:?}", obj);
    }
    pub fn trait_object_life_time_v2 () {
        trait Foo<'a> {}
        #[derive(Debug)]
        struct FooImpl<'a> {
            s: &'a [u32],
        }

        impl<'a> Foo<'a> for FooImpl<'a> {}

        fn foo<'a>(s: &'a [u32]) -> Box< dyn Foo<'a> + 'a> {
            Box::new(FooImpl { s : s})
        }
    }

}
