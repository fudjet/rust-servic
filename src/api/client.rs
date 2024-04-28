use rocket::Route;

use crate::tools::{plugins::LoginUser, res::ApiRes};

/// 获取客户的所有接口
pub fn get_api_list() -> Vec<Route> {
    routes![get_list]
}

/// 获取列表
#[get("/list")]
fn get_list(_lu: LoginUser) -> String {
    
    ApiRes::success(_lu.username, "获取成功".to_string()).to_string()
}