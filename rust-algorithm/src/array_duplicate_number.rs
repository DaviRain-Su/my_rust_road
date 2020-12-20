use std::ptr;

/**
    ex:{2, 3, 1, 0, 2, 5, 3}
    从头到尾依次扫描这个数组中的每个数字，当扫描到下表为i的数字时，首先比较这个数字（用m表示）是不是等于i。
    如果是，则接着扫描下一个数字；如果不是，则再拿它和第m个数字进行比较。如果它和第m个数字相等，就找到了一个
    重复的数字（该数字在下表为i和m的位置都出现了）；如果他和第m个数字不相等，就把第i个数字和第m个数字交换，把
    m放到属于它的位置。接下来再重复这个比较，交换的过程，直到我们发现一个重复的数字。

    以数组「2， 3， 1， 0， 2， 5， 3」为例来分析找到重复数字的步骤。数组的第0个数字（从0开始计数，和数组的
    下标保持一致）是2，与它的下标不相等，于是把它和下标为2的数字1交换。交换之后的数组是『1，3，2，0，2，5，3』
    此时第0个数字是1，仍然与它的下标不相等，继续把它和下标为1的数字3交换，得到数组『3，1，2，0，2，5，3』。
    接下来继续交换第0个数字3和第3个数字0，得到数组『0， 1， 2， 3， 2， 5 ，3』。此时第0个数字的数值为0，
    接着扫描下一个数字，在接下来的几个数字中，下标为1， 2， 3的三个数字分别为1， 2， 3，他们的下标和数值都分别
    相等，因此不需要执行任何操作。接下来扫描下标为4的数字2.由于它的数值与它的下标不等，再比较它和下标为2的数字。
    注意到此时数组中下标为2的数字也是2，也就是数字2在下标为2和下标为4的两个位置都出现了，因此找到了一个重复的数字。

*/
fn duplicate(numbers: &mut [i32], duplication: &mut i32) -> bool {
    println!("numbers length = {}", numbers.len());
    if numbers.len() == 0 {
        return false;
    }

    for &val in numbers.iter() {
        if val < 0 || val > numbers.len() as i32 {
            return false;
        }
    }

    for index in 0..numbers.len() {
        while numbers[index] != index as i32 {
            if numbers[index] == numbers[numbers[index] as usize] {
                *duplication = numbers[index];
                return true;
            }

            // swap(&numbers[index], &numbers[numbers[index]]);
            // let temp = numbers[index];
            // numbers[index] = numbers[temp as usize];
            // numbers[temp as usize] = temp;
            unsafe  {
                let x = &mut numbers[index] as *mut i32;
                let y = &mut numbers[numbers[index] as usize] as *mut i32;
                ptr::swap(x, y);
            }
        }

    }

    false
}
#[test]
fn test_duplicate() {
    let mut numbers = [2, 3, 1, 0, 2, 5, 3];
    let mut ret = 0;
    let result = duplicate(&mut numbers, &mut ret);
    println!("result = {}, ret = {}", result, ret);
}