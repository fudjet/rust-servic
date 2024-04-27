use rocket::Route;

use crate::tools::res::ApiRes;

/// 获取客户的所有接口
pub fn get_api_list() -> Vec<Route> {
    routes![get_list]
}
/// 获取列表
#[get("/list")]
fn get_list() -> String {
    ApiRes::success("".to_string(), "获取成功".to_string()).to_string()
}
