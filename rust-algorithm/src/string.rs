#[test]
fn test_string() {
    let str1 = "hello world".to_string();
    let str2 = "hello world".to_string();

    if str1 == str2 {
        println!("str1 and str2 are same.");
    } else {
        println!("str2 and str2 are not same.")
    }

    // let tmp_str = "hello world";
    let str3 = "hello world".as_ptr();
    let str4 = "hello world".as_ptr();

    if str3 == str4 {
        println!("str3 pointer is {:p}, str4 pointer is {:p}", str3, str4);
        println!("str3 and str4 are same.");
    } else {
        println!("str3 and str4 are not same.");
    }
}
