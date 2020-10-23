mod handler;

use bson::oid::ObjectId;
pub use handler::*;

#[derive(Debug)]
pub struct Article {
    // _id 是由MongoDB自动生成的，但在 文章创建前，他是不存在， 所以用Option包裹一下
    // 这个结构体不仅用于前端请求参数的接受，同时用于响应数据的返回，还用于同步数据库的模型
    _id: Option<ObjectId>,
    title: String,
    author: String,
    content: String,
}
impl Article {
    // 由于我们希望对应的表明为article，那么为Article实现一个常量字符串
    pub const TABLE_NAME: &'static str = "article";
}
