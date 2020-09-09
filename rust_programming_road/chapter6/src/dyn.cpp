// #include <concepts>
// #include <iostream>
// #include <numbers>
// #include <vector>

// using namespace std;

// //shape 

// template <typename ShapeImpl> 
// concept Shape = 
//     requires(const ShapeImpl shape) {
//         { shape.area() } -> std::same_as<double>;
//         { shape.perimeter() } -> std::same_as<double>;
//     };
// struct DynSahpe {
//     virtual auto area() const -> double { return 0; } // = 0
//     virtual auto perimeter() const -> double { return 0; }// = 0
// };

// // template <Shape ShapeImpl>
// // auto area_per_perimeter(const ShapeImpl& shape) -> double {
// // auto area_per_perimeter(const DynSahpe& shape) -> double {
// auto area_per_perimeter(const Shape auto& shape) -> double {

//     cout << "type: " << typeid(decltype(shape).name()) << "\n";
//     cout << "size: " << sizepf(decltype(shape)) << "\n";
//     return shape.area() / shape.perimeter() ;
// }

// // Cricle

// struct  CircleData
// {
//     double radius;
// };

// struct Circle: CircleData, virtual DynSahpe{
//     Circle(CircleData data): CircleData(data){}
//     auto area() const -> double override {
//         return std::numbers::pi * radius * radius;
//     }
//     auto perimeter() const -> double override {
//         return std::numbers::pi * radius * 2;
//     }
// }

// // Rectangle 

// struct RectangleData {
//     double width;
//     double height;
// };

// struct Rectangle: RectangleData, virtual DynSahpe{
//     Rectangle(RectangleData data): RectangleData(data){}
//     auto area() const -> double override {
//         return width * height;
//     }
//     auto perimeter() const -> double override {
//         return 2 * (width * height);
//     }
// }

// // main



// auto main() -> int {
//     auto c1 = Circle({.radius = 1.0});
//     auto c2 = Circle({.radius = 2.5});
//     auto r1 = Rectangle({.width = 3.0, .height = 3.0});
//     auto r2 = Rectangle({.width = 4.0, .height = 6.0});
//     cout << "c1: " << c1.area() << ", " << c2.perimeter() << "\n";
//     cout << area_per_perimeter(c1) << "\n";
//     cout << area_per_perimeter(r1) << "\n";

//     cout << "----vec----" << "\n";
//     auto circles = vector<Circle>{c1, c2};
//     auto shapes = vector<DynSahpe*>{&c1, &c2, &r1, &r2};
//     for (auto shape: shapes){
//         cout << area_per_perimeter(*shape) << "\n";
//     }
//     reutnr 0;
// }