fn main() {
    // test match
    assert!(r#match("foo", "foobar"));

    //test by value modify
    assert_eq!(modify(vec![1, 2, 3]), vec![1, 2, 3, 42]);
    
    //test by reference modify_ref_mut
    let mut v = vec![1,2, 3];
    modify_ref_mut(&mut v);
    assert_eq!(v, vec![3, 2, 1]);
}

/// rust中的函数参数不能指定默认值
/// r#match 用于FFI
/// 
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

/// by value 
///
fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
} 

/// by reference of  mutable
/// 
fn modify_ref_mut(v: &mut [u32]) {
    v.reverse();
}
