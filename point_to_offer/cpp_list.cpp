#include <iostream>

using namespace std;

struct ListNode {
    int value;
    ListNode* next;
    ListNode()
        : value(-1), next(nullptr){}
    ListNode(int value)
        : value(value), next(nullptr) {
            cout << "ListNode" << endl;
        }
};

void print_list(ListNode** pHead){
    ListNode* pTemp = *pHead;
    while (pTemp->next != nullptr)
    {
        cout << pTemp->value << " ";
        pTemp = pTemp->next;
    }
    cout << endl;
}
void add_to_tail(ListNode** pHead, int value){
    // cout << value << endl;
    ListNode* new_node = new ListNode(value);

    if(*pHead == nullptr){
        // cout << "***********" << endl;
        *pHead = new_node;
    }else {
        // cout << "-----------" << endl;
        ListNode* pNode = *pHead;

        while(pNode->next != nullptr){
            pNode = pNode->next;
            // cout << pNode->value << endl;
        }
        pNode->next = new_node;
    }
}

int main() {
    ListNode *phead1 = new ListNode();
    ListNode ** phead = &phead1;

    // for(int i = 0; i < 10; i++){
    add_to_tail(phead, 0);
    add_to_tail(phead, 1);
    add_to_tail(phead, 2);
    add_to_tail(phead, 3);
    add_to_tail(phead, 3);
    add_to_tail(phead, 3);
    add_to_tail(phead, 3);

    // }

    print_list(phead);
}
