// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
//
// 示例1:
//
// 输入: 123
// 输出: 321
//
// 示例 2:
//
// 输入: -123
// 输出: -321
//
// 示例 3:
//
// 输入: 120
// 输出: 21
// 注意:
// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为[−2^31，2^31− 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

fn reverse(x: i32) -> i32 {

    let str_x : String = x.abs().to_string().chars().rev().collect();

    if x.is_negative() {
        str_x.parse::<i32>().unwrap_or(0) * -1
    } else {
        str_x.parse::<i32>().unwrap_or(0)
    }

}

#[test]
fn test_reverse() {
    let num = 1534236469;
    let ret = reverse(num);
    println!("ret = {}", ret);
    let num = -123;
    let ret = reverse(num);
    println!("ret = {}", ret);
}