#include <iostream>
#include <stdlib.h>
#include <thread>
#include <string>


#define COUNT 1000000

volatile int g_num = 0;

void thread_1() {
    for (int i = 0; i < COUNT; i++) {
        g_num++;
    }
}

void thread_2() {
    for (int i =0; i < COUNT; i++) {
        g_num--;
    }
}

int main()
{
    std::thread t1(thread_1);
    std::thread t2(thread_2);

    t1.join();
    t2.join();
    std::cout << "final value :" << g_num << std::endl;
    return 0;
}

