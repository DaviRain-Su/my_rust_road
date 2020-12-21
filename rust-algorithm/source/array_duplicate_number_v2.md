# 题目二: 不修改数组找出重复的数字

## 题目

在一个长度为n+1的数组里的所有数字都在1~n的范围内，所以数组中至少有一个数字是重复的，请找出数组中任意一个重复的数字。
但不能修改输入的数组。例如，如果输入长度为8的数组[2, 3, 5, 4, 3, 2, 6, 7],那么对应的输出是重复的数字2或者3.

## 题解

假设没有重复的数字，那么在从1 ~ n 的范围里只有n个数字。由于数组里包含超过n个数字，所以一定包含了重复的数字。
那么看起来在某范围里数字的个数对解决这个问题很重要。

我们把从1 ~ n 的数字从中间的数字m分为量部分，前一半为 1 ~ m，后一半为 m+1 ~ n。 如果1 ~ m的数字的数目超过m，
那么这一半的区间里一定包含重复的数字；否则，另一半m+1 ~ n的区间里一定包含重复的数字。我们可以继续把包含重复的数字的区间一分为二，
直到找到一个重复的数字。这个过程和二分查找很类似，只是多了一步统计区间里数字的数目。

例如以长度8的数组[2, 3, 5, 4, 3, 2, 6, 7]为例分析查找的过程。根据题目要求这个长度为8的所有数字都在1~7的范围内。中间的数字4把
1~7的范围分为了两段，一段是1 ~ 4 ，另一段是5 ~ 7.接下来统计 1 ~ 4这4个数字在数组中出现的次数，他们一共出现了5次，因此这4个数字中
一定有重复的数字。

接下来再把1 ~ 4的范围一分为二，一段是1，2两个数字，另一段是3，4两个数字。数字1或者2在数组中一共出现了两次，我们再统计3，4在数组中出现的次数。
他们一共出现了三次。这意味着3，4；两个数字中一定有重复了。我们再分别统计这两个数字在数组中出现的次数，接着我们发现数字3出现了两次，是一个重复的数字。

## Code(Rust)

```rust
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
    // println!("number len = {}", numbers.len());
    if numbers.len() == 0 {
        return -1;
    }

    let mut start = 1;
    let mut end = (numbers.len() - 1) as i32;

    while start <= end {
        let middle = ((end - start) >> 1) + start;
        let count = count_range(numbers, start, middle);
        // println!("count = {}", count);
        if start == end {
            // println!("start = {}, end = {}", start, end);
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
    // let arr = [0, 1, 0]; TODO: Issues
    let ret = get_duplication(&arr);
    println!("ret = {}", ret);
}
```

## Code(C)
## Code(Go)
## Code(Python3)
## Code(C++)
## Code(Java)
