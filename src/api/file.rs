use rocket::data::ToByteUnit;
use rocket::{Data, Route};

use crate::tools::file::FormFile;
use crate::tools::res::ApiRes;

/// 文件上传
#[post("/upload", format = "multipart/form-data", data = "<file>")]
async fn upload(file: Data<'_>) -> String {
    let x = file
        .open(500.mebibytes())
        .into_bytes()
        .await
        .unwrap()
        .into_inner();
    if x.len() < 10 {
        return "文件不存在".to_string();
    }
    match FormFile::new_save(x) {
        Ok(ff) => ApiRes::success(ff.path, "上传成功!".to_string()).to_string(),
        Err(e) => ApiRes::error(e, "上传失败!".to_string()).to_string(),
    }
}
pub fn get_api_list() -> Vec<Route> {
    routes![upload]
}
// let filename = "E:/code/test.jpg";
