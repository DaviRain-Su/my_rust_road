#include <iostream>
#include <memory>
using namespace std;

int main()
{
    unique_ptr<int> orig(new int(5));
    cout << "*orig is " << *orig << endl;
    cout << "orig address is " << orig << endl;
    auto stolen = move(orig);
    cout << "*stolen is " << *stolen << endl; 
    cout << "stolen address is " <<stolen << endl;
    //cout << *orig << endl; // segmentation falult 对空指针解引用
    return 0;
}

