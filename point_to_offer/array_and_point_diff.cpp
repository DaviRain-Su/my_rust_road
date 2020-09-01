#include <iostream>

using namespace std;


/// 区分C++中数组和指针，C++没有记录数组的大小，因此在用指针访问数组中的元素时，需要保证数组没有越界
/// 
/// 代码区分数组和指针的区别
int get_size(int *data){
    return sizeof(data);
}

int main() {
    int data_1[] = {1, 2, 3, 4, 5};
    int size_1 = sizeof(data_1);

    int *data_2 = data_1;
    int size_2 = sizeof(data_2);

    int size_3 = get_size(data_1);

    cout << "size_1 = " << size_1 << ", size_2 = " << size_2 << ", size_3 = " << size_3 << endl;

    return 0;

}