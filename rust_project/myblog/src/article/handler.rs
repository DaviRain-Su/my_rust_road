use super::Article;
use crate::article::Article;
use crate::collection;
use crate::common::*;
use actix_web::{web, HttpRequest, HttpResponse};
use bson::oid::ObjectId;
use bson::ordered::OrderedDocument;
use bson::Document;
use log::*;
use serde::{Deserialize, Serialize};

type SimpleResp = Result<HttpResponse, BusinessError>;

pub fn save_article(article: web::Json<Article>) -> SimpleResp {
    let article: Article = article.into_inner();

    info!("save article, {:?}", article);
    Resp::ok(article.title).to_json_result()
}
