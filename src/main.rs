use tools::plugins::{get_cors, get_error_list};

#[macro_use]
extern crate rocket;

// 自己的
mod api;
pub mod db;
pub mod tools;

#[launch]
fn rocket() -> _ {
    // 链接本地数据库 sqlite
    let r = rocket::build().attach(db::stage()).attach(get_cors());

    // .attach(Logs::init())
    // .mount("/", routes![read]);
    // 处理 错误
    let r = r.register("/", get_error_list());
    // 加载 路由
    let r = api::api_routes(r);
    r
}
