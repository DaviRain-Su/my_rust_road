fn count_range(numbers: &[i32], start: i32, end: i32) -> i32 {
    println!("len is {}", numbers.len());
    if numbers.len() == 0 {
        return 0;
    }
    let mut count = 0;
    for i in 0..numbers.len() {
        if start <= numbers[i] && numbers[i] <= end {
            count += 1;
        }
    }
    count
}

fn get_duplication(numbers: &[i32]) -> i32 {
    println!("number len = {}", numbers.len());
    if numbers.len() == 0 {
        return -1;
    }

    let mut start = 1;
    let mut end = (numbers.len() - 1) as i32;

    while start <= end {
        let middle = ((end - start) >> 1) + start;
        let count = count_range(numbers, start, end);
        println!("count = {}", count);
        if start == end {
            println!("start = {}, end = {}", start, end);
            if 1 < count {
                return start;
            } else {
                break;
            }
        }

        if (middle - start + 1) < count {
            end = middle;
        } else {
            start = middle + 1;
        }
    }
    -1
}

#[test]
fn test_duplicate() {
    let arr = [2, 3, 5, 4, 3, 2, 6, 7];
    let ret = get_duplication(&arr);
    println!("ret = {}", ret);
}