#include <vector>
#include <algorithm>

namespace davirain {


/// print vector<T>
template <typename T>
void print_vector(const std::vector<T> &vecs) {
    std::for_each(vecs.begin(), vecs.end(), [](const T& value) {
        std::cout << value << " ";
    });
    std::cout << std::endl;
}

/// overload print vector<T>
template <typename T>
void print_vector(const std::vector<T> &&vecs) {
    print_vector(vecs);
}

}// end of namespace davirain