// #![warn(unused_variables)]

/// Unsafe Rust
/// 五中不同的环境
/// - 解引用裸指针
/// - 调用unsafe函数或方法
/// - 访问或者修改可变静态变量
/// - 实现unsafe triat
/// - 读写Union联合体字段
///
#[test]
fn test_unsafe_hello_world() {
    unsafe {
        let mut a = "hello";
        let _b = a;
        let _c = &mut a;
    }

    let hello = vec![104, 101, 108, 108, 111];
    let hello = unsafe { String::from_utf8_unchecked(hello) };

    println!("hello = {}", hello);
    assert_eq!("hello", hello);
}

#[test]
fn test_static_var() {
    static mut COUNTER: u32 = 0;
    let inc = 3;
    unsafe {
        COUNTER += inc;
        println!("COUUNTER: {}", COUNTER);
    }
}

#[test]
fn test_union_type() {
    #[repr(C)]
    union U {
        i: i32,
        f: f32,
    }

    #[repr(C)]
    struct Value {
        tag: u8,
        value: U,
    }

    #[repr(C)]
    union MyZero {
        i: Value,
        f: Value,
    }

    enum MyEnumZero {
        I(i32),
        F(f32),
    }

    let int_0 = MyZero {
        i: Value {
            tag: b'0',
            value: U { i: 0 },
        },
    };

    let float_0 = MyZero {
        f: Value {
            tag: b'1',
            value: U { f: 0.0 },
        },
    };
}
