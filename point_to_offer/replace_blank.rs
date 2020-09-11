// fn replace_blank(s: String) -> String { 
//     let blank_count = s.chars().fold(0, |mut acc, x| {
//         if x == ' ' {
//             acc += 1;
//         }
//         acc
//     });
//     println!("blanck count = {}", blank_count);
//     println!("s size = {}", s.len());
//     println!("s capity = {}", s.capacity());
//     let mut s = s;
//     s.reserve(22);
//     println!("s size = {}", s.len());
//     println!("s capity = {}", s.capacity());

//     let new_len = s.len() + blank_count * 2;
    
//     let mut index_orig = s.len();
//     let mut index_new = new_len;
//     loop {
//         if index_new >= 0 && index_new > index_orig {
//             if s[index_orig] == ' ' {
                
//                 s[index_new] = '0';
//                 index_new -= 1;
//                 s[index_new] = '2';
//                 index_new -= 1;
//                 s[index_new] = '%';
//                 index_new -= 1;
//             }else {
//                 s[index_new] = s[index_orig];
//                 index_new -= 1;
//             }
//             index_orig -= 1;
//         }
//     } 
//     s
// }
// 上面好像用rust搞不了

fn replace_space(s: String) -> String {
    let mut result = String::new();
    for val in s.chars() {
        if val == ' '{
            result.push_str("%20");
        }else{
            result.push(val);
        }
    }
    result
}

fn main() {
    let s = String::from("we are happy");
    let ret = replace_space(s);
    println!("ret = {}", ret);
}