// replace blank
fn replace_blank(mut par_str: String) -> String {
    if par_str.len() == 0 {
        return "".to_string();
    }

    // blank num
    let number_of_blank = par_str
        .chars()
        .filter(|&char| char == ' ')
        .count();

    let original_length = par_str.len();
    let new_length = original_length + number_of_blank * 2;

    // for append par_str num
    let number_blank = number_of_blank * 2;


    let mut index_of_original = original_length - 1;
    let mut index_of_new = new_length - 1;

    // change String to vec<char> for index
    let mut par_str: Vec<char> = par_str.chars().map(|val| val ).collect();

    // append blank to par_str
    let mut temp = vec![' ';number_blank];
    par_str.append(&mut temp);// append empty char

    while index_of_original < index_of_new {
        if par_str[index_of_original] == ' ' {
            par_str[index_of_new] = '0';
            index_of_new -= 1;
            par_str[index_of_new] = '2';
            index_of_new -= 1;
            par_str[index_of_new] = '%';
            index_of_new -= 1;
        } else {
            par_str[index_of_new] = par_str[index_of_original];
            index_of_new -= 1;
        }
        index_of_original -= 1;
    }

    let par_str = par_str.iter().map(|val| val).collect();
    par_str
}

#[test]
fn test_replace_blank() {
    let s = "we are happy".to_string();

    let ret = replace_blank(s);
    println!("********ret******** = {}", ret);
}
