// 在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序。请完成一个函数，输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。

 

// 示例:

// 现有矩阵 matrix 如下：

// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]

// 给定 target = 5，返回 true。

// 给定 target = 20，返回 false。

 

// 限制：

// 0 <= n <= 1000

// 0 <= m <= 1000

/// 解题规律：
/// 首先选取数组中右上角的数字。
/// 如果该数字等于要查找的数字， 则查找过程结束；
/// 如果该数字大于要查找的数字，则删除这个数字所在的列；
/// 如果该数字小于要查找的数字，则删除这个数字所在的行。
/// 也就是说要查找的数字不再数组右上角，则每一次都在数组的查找范围中删除一行或者一列，
/// 这样每一步都可以缩小查找的范围，直到找到要查找的数字，或者查找范围为空。
/// 
fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    if rows == 0 { return false; }
    let columns = matrix[0].len();
    if columns == 0 { return  false;}

    let mut found = false;
    
    let mut row = 0;
    let mut column = columns - 1;

    while row < rows && column >= 0 {
        if matrix[row][column] == target {
            found = true;
            break;
        }else if matrix[row][column] > target {
            if column == 0 {
                break;
            }
            column -= 1;
        }else {
            row += 1;
        }
    }

    return found;
}


fn main() {

    let matrix = vec![
                      vec![1, 2, 8, 9],
                      vec![2, 4, 9, 12],
                      vec![4, 7, 10, 13],
                      vec![6, 8, 11, 15]
                     ];
    let ret =  find_number_in2_d_array(matrix, 5);
    println!("ret = {}", ret);
}
