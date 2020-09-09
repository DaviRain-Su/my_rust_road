


// pub struct PostingsList {
//     head: NodeNext,
// }

// type NodeNext = Option<Box<Node>>;

// struct Node {
//     document_id: i32,
//     positions: Vec<i32>,
//     positions_count: i32,
//     next: NodeNext,
// }
// use std::collections::HashMap;
// pub struct InvertedIndexHash {
//     tokeb_id: i32,
//     postings_list: PostingsList,
//     docs_count: i32,
//     positions_count: i32,
//     hh : HashMap<i32, i32>,// TODO
// }
// pub enum CompressMethod {
//     CommpressNone, // 不压缩
//     CompressGolomb, // 使用Golomg 编码
// }
// pub struct WiserEnv {
//     db_path: String,
//     token_len: i32,
//     compress: CompressMethod,
//     enable_phrase_search: i32,
//     ii_buffer: Vec<InvertedIndexHash>,
//     ii_buffer_count : i32,
//     ii_buffer_update_threshold: i32,
//     indexed_count : i32,

//     // sqlit3配置
// }
// // #[link(name = "expat")]
// // extern "C" fn XML_ParserCreate(ending : *const XML_Char) -> XML_Parser;

// pub mod xml_parser {
//     use super::*;
//     pub enum WikipedaStatus {
//         // 以下几种状态以外的状态
//         InDocument,
//         // 位于<page> 标签中
//         InPage,
//         // 位于<page>标签中的<title> 标签中
//         InPageTitle,
//         // 位于<page>标签中的<id>标签中
//         InPageId,
//         // 位于<page> 标签中的<revision> 标签中
//         InPageRevision,
//         // 位于<page> 标签中的<revision> 标签中的<text>标签中
//         InPageRevisionText,
//     }

//     pub struct WikipedaParser <T> 
//         where T: Fn(u32) -> u32
//     {
//         env: WiserEnv,
//         status : WikipedaStatus,
//         title: String,
//         body: String,
//         article_count: u32,
//         max_article_count: u32,
//         add_document_callback: T,
//     }

    

//     impl<T> WikipedaParser<T> 
//         where T: Fn(u32) -> u32 
//     {
//         pub fn new(env: WiserEnv, add_document_callback: T, max_article_count: u32) -> Self {
//             Self {
//                 env,
//                 status: WikipedaStatus::InDocument,
//                 title: String::new(),
//                 body: String::new(),
//                 article_count: 0,
//                 max_article_count,
//                 add_document_callback,
//             }
//         }


//         fn load_wikipedia_dump(&mut self, path: &str, add_document_callback: T, max_article_count: i32) -> i32 {
            


//             0
//         }
//     }
// }


