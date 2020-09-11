#include <iostream>
#include <string>
#include <algorithm>

using namespace std;

void replace_space(string& temp) {
    int temp_capity = temp.capacity();
    if (temp.size() == 0) {
        return;
    }

    // origin length 是字符串的实际长度
    int origin_len = temp.size();
    int number_of_blank = 0;
    for_each(temp.begin(), temp.end(), [&number_of_blank](const char& val){
        if (val == ' ') {
            ++number_of_blank;
        }
    });
    
    int new_len = origin_len + number_of_blank * 2;
    temp.resize(new_len);

    int index_of_orign = origin_len;
    int index_of_new = new_len;
    while(index_of_orign >= 0 && index_of_new > index_of_orign){
        if (temp[index_of_orign] == ' '){
            temp[index_of_new--] = '0';
            temp[index_of_new--] = '2';
            temp[index_of_new--] = '%';
        }else {
            temp[index_of_new--] = temp[index_of_orign];
        }
        --index_of_orign;
    }
}

string replace(string temp){
    string result;
    for_each(temp.begin(), temp.end(), [&result](const char &val){
        if(val == ' '){
            result.push_back('%');
            result.push_back('2');
            result.push_back('0');
        }else{
            result.push_back(val);
        }
    });
    return result;
}

int main() {
    string temp{"we are happy"};
    cout << "before = " << temp << endl;
    replace_space(temp);
    cout << "after = " << temp << endl;
    return 0;
}