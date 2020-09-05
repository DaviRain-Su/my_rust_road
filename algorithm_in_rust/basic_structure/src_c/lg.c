#include <stdio.h>

int lg(int);

int main() {
    for(int i = 1, N = 10; i <= 6; i++, N *= 10){
        printf("%7d %2d %9d\n", N, lg(N), N*lg(N));
    }
    return 0;
}

// log_2(N), 求以2为底的对数
int lg(int N){ 
    int i;
    for (i = 0; N > 0; i++, N /= 2){
        // printf("%d %d ", N, i);
    }
    // printf("%d %d\n", N, i);
    return i;
}