// 自己的
mod api;
pub mod db;
pub mod tools;
// 必要的
use rocket::Request;
#[macro_use]
extern crate rocket;

// 服务器错误
#[catch(500)]
fn server_error() -> String {
    "{msg:'500，服务器错误'}".to_string()
}
// 数据请求错误，可能是缺少参数
#[catch(400)]
fn bad_request(req: &Request<'_>) -> String {
    println!("{:?}", req);
    "{msg:'400，请求错误'}".to_string()
}
#[launch]
fn rocket() -> _ {
    // 链接本地数据库 sqlite

    let r = rocket::build().attach(db::stage());
    // .attach(Logs::init())
    // .mount("/", routes![read]);
    // 处理 错误
    let r = r.register("/", catchers![server_error, bad_request]);
    // 加载 路由
    let r = api::api_routes(r);
    r
}
