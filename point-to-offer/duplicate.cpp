#include <iostream>
#include <vector>
#include "lib/utils.hpp"

using namespace std;
using namespace davirain;

/*

找出数组中重复的数字
在一个长度为n的数组里的所有数组都在0 ~ n-1 的范围内。数组中的某些数字书重复的，但不知道有几个重复了，
也不知道每几个数字重复了几次，请找出数组任意一个重复的数字。
例如， 如果输入了长度为7的数组{2, 3, 1, 0, 2, 5, 3}, 那么对应的重复数字2或者3.

*/

/// this function will change numbers,
auto duplicate(vector<int>& numbers, int &duplication) -> bool {
    size_t len = numbers.size();
    if ( len <= 0 ) return false;

    for_each(numbers.begin(), numbers.end(), [len](const int &value) {
        if(value < 0 || value > len - 1) return false;     
    });

    for (auto i = 0; i < len; i++){
        while (numbers[i] != i) {
            if (numbers[i] == numbers[numbers[i]]){
                duplication = numbers[i];
                return true;
            }else {
                swap(numbers[i], numbers[numbers[i]]);
            }
        }
    }
    return true;
}

/*

不修改数组找出重复的数字

在一个长度为n + 1的数组里的所有数字都在 1 ~ n的范围内，所以数组中至少有一个数字是重复的，
请找出数组中任意一个重复的数字， 但不能修改输入的数组。例如， 如果输入长度为8的数组{2, 3, 5, 4, 3, 2, 6, 7},
那么对应的输出是重复数字2或者3.

*/

auto get_duplication(const vector<int> &numbers) -> int {
     auto count_range = [](const vector<int> numbers, int start, int end){
         cout << "numbers len = " << numbers.size() << endl;
         int count = 0;
         std::for_each(numbers.begin(), numbers.end(), [start, end, &count] (const int &value){
            if (start <= value && value <= end) ++count;
         });
         return count;
    };

    size_t len = numbers.size();    
    if (len <= 0) return -1;

    int start = 1;
    int end = len - 1;
    while (start <= end)
    {
        int middle = ((end - start) >> 1) + start;

        int count = count_range(numbers, start, middle);
        cout << "count = " << count << ", start = " << start << ", middle = " << middle  << endl;
        if (end == start){
            if (count > 1) return start;
            else break;
        }

        if (count > (middle - start  + 1)) end = middle;
        else start = middle + 1;
    }
    return -1;
}


int main() {
    vector<int> vecs{2, 3, 1, 0, 2, 5, 3};
    int temp = 0;

    bool ret = duplicate(vecs, temp);
    cout << "bool = " << ret << ", temp = " << temp << endl;
    print_vector(vecs);
    print_vector(vector<int>{1,2 , 3, 4, 5});

    vector<int> vec{2, 3, 5, 4, 3, 2, 6, 7};
    int ret_1 = get_duplication(vec);
    cout << "ret_1 = " << ret_1 << endl;
    print_vector(vec);
    return 0;
}