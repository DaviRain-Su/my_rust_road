fn main() {
    let str1  = "hello, world";
    let str2  = "hello, world";

    // let ref ref_str1 = str1;
    // let ref ref_str2 = str2;

    println!("ref str1 = {:p}, ref srt2 = {:p}", &str1, &str2);

    let str1_ptr = str1.as_ptr();
    let str2_ptr = str2.as_ptr();

    println!("str1 address = {:p}, str2 address = {:p}", str1_ptr, str2_ptr);
    
    let str3 = String::from("hello, world");
    let str4 = String::from("hello, world");

    println!("str3 address = {:p}, str3 address = {:p}", str3.as_ptr(), str4.as_ptr());
    
    // let ref ref_str3 = str3;
    // let ref ref_str4 = str4;

    println!("ref str3 = {:p}, ref srt4 = {:p}", &str3, &str4);

    assert_eq!(str1,str2);
    assert_eq!(str1.as_ptr(), str2.as_ptr());
    assert_eq!(str3, str4);
    // assert_eq!(str3.as_ptr(), str4.as_ptr()); // != 
}