## FFI 惯用法

编写FFI 代码本省就是一门完整的课程。但是这里有几个惯用法可以作为指导方针，并且避免不熟练的用户使用unsafe Rust.

这部分包含在使用FFI时可能有用的惯用法

- 惯用法错误（Idiomatic Errors）用整数代码和定点返回值(如NULL指针)处理错误
- 接受（Accepting Strings）带有最小不安全代码的字符串
- 将字符串传递给FFI函数（Passing Strings）



