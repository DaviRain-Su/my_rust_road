#include <iostream>
#include <iomanip>

using std::cout;
using std::endl;
using std::setw;

int lg(int);
int main() {
    for(int i = 1, N = 10; i <= 6; i++, N *= 10){
        // printf("%7d %2d %9d\n", N, lg(N), N*lg(N));
        cout << setw(7) << N << " " << setw(2) << lg(N) << " " << setw(9) << N*lg(N) << endl;
    }
    return 0;
}

int lg(int N){
    int i;
    for(i = 0; N > 0; i++, N /= 2) ;
    return i;
}