#include <iostream>
#include <vector>
#include "lib/utils.hpp"
#include <cassert>

using namespace std;
using namespace davirain;

// 从右上角开始查找
auto find_righ_tri(const vector<vector<int>> &matrix, int target) -> bool{
    auto rows =  matrix.size();
    // assert(rows != 0);
    if (rows == 0) { return false; }    
    auto columns = matrix[0].size();
    // assert(columns != 0);
    if (columns == 0) { return false; }

    bool found = false;

    int row = 0;
    int column = columns - 1;

    while(row < rows && column >= 0) {
        cout << "row = " << row << ", column = " << column << endl;
        if (matrix[row][column] == target) {
            found = true;
            break;
        }else if (matrix[row][column] > target){
            --column;
        }else {
            ++row;
        }
    }
    return found;
}

// 从左下角开始
auto find_left_tri(const vector<vector<int>>& matrix, int target) {
    auto rows = matrix.size();
    if (rows == 0) { return false; }
    auto columns = matrix[0].size();
    if (columns == 0) { return false; }

    bool found = false;

    int row = rows - 1;
    int column = 0;

    while (row >= 0 && column < columns)
    {
        cout << "row = " << row << ", column = " << column << endl;
        if(matrix[row][column] == target){
            found = true;
            break;
        }else if(matrix[row][column] < target){
            ++column;
        }else {
            --row;
        }
    }
    return found;
}


int main() {

    vector<vector<int>> matrix{
                                {1, 2, 8, 9},
                                {2, 4, 5, 12},
                                {4, 7, 10, 13},
                                {6, 8, 11, 15}
                              };
    // vector<vector<int>> matrix{};
    // vector<vector<int>> matrix{{}};
    auto ret = find_left_tri(matrix, 7);
    // cout << std::boolapha // print bools as true or false
    cout << std::boolalpha <<  "ret = " << ret << endl;
    return 0;
}