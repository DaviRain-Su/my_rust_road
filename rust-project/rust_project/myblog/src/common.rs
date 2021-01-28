use actix_web::{HttpResponse, error};
use serde::{Serialize, Deserialize};
use failure::Fail;

#[derive(Deserialize, Serialize)]
pub struct Resp<T>
where
    T: Serialize,
{
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_owned(),
            data: Some(data),
        }
    }
    pub fn to_json_result(&self) -> Result<HttpResponse, BusinessError> {
        Ok(HttpResponse::Ok().json(self))
        }
    }
}

impl Resp<()> {
    pub fn err(error: i32, message: &str) -> Self {
        Self {
            code : error,
            message: message.to_owned(),
            data: None
        }
    }
}

#[derive(Fail, Debug)]
pub enum BusinessError {
    // ValidationError表示请求参数校验错误
    #[fail(display = "Validation error on field : {}", field)]
    ValidationError { field: String },
    // InternalError 作为普通内部错误
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            BusinessError::ValidationError{ .. } => {
                let resp = Resp::err(10001, &self.to_string());
                HttpResponse::BadRequest().json(resp)
            }
            _ => {
                let resp = Resp::err(10000, &self.to_string());
                HttpResponse::InternalServerError().json(resp)
            }
        }
    }
    // 重写response 的序列化结果
    fn render_response(&self) -> HttpResponse {
        self.error_response()
    }
}
