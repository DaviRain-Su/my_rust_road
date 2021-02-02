# 简单的文档初始化

## 描述

如果一个结构体的初始化需要耗费大量的精力，在编写文档时，用一个以结构为参数的辅助函数来包装你的例子会更快。

## 动机

有时，一个结构体有多个或者复杂的参数和几个方法，这些方法中每一个都应该有例子。

```rust
struct Connection {
  name: String,
  stream: TcpStream,
}
impl Connection {
  /// Sends a request over the connection. 
 	/// 
  /// # Example 
  /// ```no_run
  /// # // Boilerpalte are required to get an example working. 
  /// # let stream = TcpStream::connection("127.0.0.1:34254");
  /// # let conection = Connection { name: "foo".to_owned(), stream};
  /// # let request = Request::new("RequestId", RequestType::Get, "payload");
  /// let response = connection.send_request(request);
  /// assert!(response.is_ok());
  /// ```
 	fn send_request(&self, request: Request) -> Result<Status, SendErr> {
    // ...
  }
  
  ///Oh no, all that boilerpalte needs to be repeated here! 
  fn chech_status(&self) -> Status {
    // ...
  }
}	
```



## 例子

与其键入所有这些模板来创建一个`Connection` 和`Request`, 不如直接创建一个将他们作为参数的封装帮助函数`

```rust
struct Connection {
  name: String,
  stream: TcpStream,
}

impl Connection {
  /// Sends a request over the connection.
  ///
  /// # Example 
  /// ```
  /// # fn call_send(connection: Connection, request: Request) {
  /// 	let response = connection.send_request(request);
  /// 	assert!(reponse.is_ok());
  /// #}
  /// ```
  fn send_request(&self, request: Request) {
    // ... 
  }
}
}
```

Note: 在上面的例子中	`assert!(response.is_ok());`	 这一行在测试时不会实际运行，以为他是在一个未被调用的函数里面。

## 优点

这样更简洁，并避免了示例中的重复代码

## 缺点

由于例子是在一个函数中，代码不会被测试。虽然在运行`cargo test`时，然后会检查它是否被编译。所以这个模式在需要`no_run`的时候是最拥有的。有了这个模式，你就不需要添加`no_run`了。

## 讨论

如果不需要断言，则此模式工作的很好。

如果是的话，一个替代的方法是创建一个公共方法来创建一个帮助实例，这个实例用`#[doc(hidden)]`来注释，（这样用户就看不到它了）。然后这个方法可以在rustdoc里面调用，因为他是create的公共API的一部分。