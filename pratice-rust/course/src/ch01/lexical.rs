#![allow(unused_variables)]
#![allow(dead_code)]
//! 第一章： Rust 语言基础
//! 1.3 语法面面观（一） 词法结构
//!
//! 包括：
//!     1. Rust 语言版本说明
//!     2. Rust 词法结构

/**
    # 标识符

    ```rust
    let thinking = "thinking";
    let thinking123_ = "thinking 123";

    //error : invalid suffix "thinking" for integer literal
    // let 321_thinking = "thinking";

    //ok
    let _321_thinking = "thinking";

    // non-ascii ident
    // RFC: https://github.com/rust-lang/rfcs/blob/master/text/2457-non-ascii-idents.md
    // error: unknown start of token :\u{1f914}
    // let 😁 = "thinking ";
    ```
*/
pub fn ident_show() {
    let thinking = "thinking";

    // non-ascii ident
    // RFC: https://github.com/rust-lang/rfcs/blob/master/text/2457-non-ascii-idents.md
    // error: unknown start of token :\u{1f914}
    // let 😁 = "thinking ";
}

/**

    pub mod outer_module {
        //! -模块级文档注释，置于模块头部
        //!！ - 模块级文档注释，但是和上面注释置于同一行


        //! - 模块级文档注释，但会换行
        /*！ - 模块级文档注释 */
        /*!! - 模块级文档注释，但是和上面注释置于同一行 */

        /*! - 模块级注释，但会换行 */

        // - 普通行注释
        /// 行级文档注释(必须是三个斜杠）
        ///// - 普通行注释

        /* - 普通块级注释 */
        /** - 块级文档注释2个星号 */
        /*** - 普通注释 */

        pub mod inner_module {}

        /// mod定义个模块
        pub mod nested_comments {
            /* Rust 中的注释 /* 可以 /* 嵌入注释 */*/*/

            // 所有三种注释可以相互嵌套

            /* /* */ /** */ /*! */ */
            /*! /* */ /** */ /*!*/ */
            /** /* */ /** */ /*! */ */
            pub mod dummy_item {}
        }

        pub mod degenerate_cases {
            // 空的模块级文档注释
            //!

            // 空的模块级文档注释
            /*! */

            // 空的行注释
            //

            // empty outer lin doc
            /// 空的行级文档注释

            // 空的块注释
            /**/


            // pub mod dummy_item {}

            // 注意，此处不是空的块级文档注释，而只是一个普通块级注释
            /***/
        }


        /*
        下面这种注释是不允许的， 因为文档注释下面必须要有语言项，比如方法，函数等
        /// where is my item?
        */
    }

*/
pub mod outer_module {
    //! -模块级文档注释，置于模块头部
    //!！ - 模块级文档注释，但是和上面注释置于同一行

    //! - 模块级文档注释，但会换行
    /*！ - 模块级文档注释 */
    /*!! - 模块级文档注释，但是和上面注释置于同一行 */

    /*! - 模块级注释，但会换行 */

    // - 普通行注释
    /// 行级文档注释(必须是三个斜杠）
    ///// - 普通行注释

    /* - 普通块级注释 */
    /** - 块级文档注释2个星号 */
    /*** - 普通注释 */

    pub mod inner_module {}

    /// mod定义个模块
    pub mod nested_comments {
        /* Rust 中的注释 /* 可以 /* 嵌入注释 */*/*/

        // 所有三种注释可以相互嵌套

        /* /* */ /** */ /*! */ */
        /*! /* */ /** */ /*!*/ */
        /** /* */ /** */ /*! */ */
        pub mod dummy_item {}
    }

    pub mod degenerate_cases {
        // 空的模块级文档注释
        //!

        // 空的模块级文档注释
        /*! */

        // 空的行注释
        //

        // empty outer lin doc
        /// 空的行级文档注释

        // 空的块注释
        /**/

        pub mod dummy_item {}

        // 注意，此处不是空的块级文档注释，而只是一个普通块级注释
        /***/
    }

    /*
    下面这种注释是不允许的， 因为文档注释下面必须要有语言项，比如方法，函数等
    /// where is my item?
    */
}

/**
    # 声明宏

    ```rust
    pub fn macro_show() {
        macro_rules! calculate {
            (eval $e:expr) => {{
                {
                    let val : usize = $e; // force types to be integers
                    println!("{} = {}", stringify!{$e}, val);
                }
            }};
        }

        calculate! {
            eval 1 + 2 // hehehe 'eval' is not Rust keywords
        }

        calculate!{
            eval (1 + 2) * (3 / 4)
        }
    }
    ```
*/

pub fn macro_show() {

    macro_rules! calculate {
        (eval $e:expr) => {{
            {
                let val : usize = $e; // force types to be integers
                println!("{} = {}", stringify!{$e}, val);
            }
        }};
    }

    calculate!{
        eval 1 + 2 // hehehe 'eval' is not Rust keywords
    }

    calculate!{
        eval (1 + 2) * (3 / 4)
    }
}


/**
    #Path

    ```rust

    // 模块路径
    mod a {
        fn foo() {}

        mod b {
            mod c {
                fn foo() {
                    super::super::foo(); // call a's foo function
                    self::super::super::foo(); // call a's foo function
                }
            }
        }
    }

    // 方法调用

    struct  S;
    impl S {
        fn f() {
            println!("S");
        }
    }
    trait T1 {
        fn f() {
            println!("T1 f");
        }
    }

    impl T1 for S {}
    trait T2 {
        fn f() {
            println!("T2 f");
        }
    }

    impl T2 for S {}
    S::f(); // calls the inherent impl
    // 完全限定无歧义调用
    <S as T1>::f(); // calls the T1 trait function.
    <S as T2>::f(); // calls the T2 trait function.


    // 泛型函数-turbofish 操作符
    (0..10).collect::<Vec<_>>();
    Vec::<u8>::with_capacity(1024);
    ```
*/
pub fn path_show() {
    let unused_var = (0..10).collect::<Vec<_>>();
    Vec::<u8>::with_capacity(1024);
}
