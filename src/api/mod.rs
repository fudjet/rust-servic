mod client;
mod file;
mod user;

/// api 路由
pub fn api_routes(r: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    // 用户api
    r.mount(set_api_path(""), user::get_api_list())
        // 文件api
        .mount(set_api_path("file"), file::get_api_list())
        // 客户api
        .mount(set_api_path("client"), client::get_api_list())
}

fn set_api_path(p: &'static str) -> String {
    return format!("/api/{}", p.to_owned());
}
