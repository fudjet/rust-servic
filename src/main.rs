#[macro_use]
extern crate rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

// 必要的
use rocket::{http::Method, Request};

// 自己的
mod api;
pub mod db;
pub mod tools;

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
// 跨域
fn get_cors() -> Cors {
    let allowed_origins = AllowedOrigins::All;
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();
    cors
}

#[launch]
fn rocket() -> _ {
    // 链接本地数据库 sqlite
    let r = rocket::build().attach(db::stage()).attach(get_cors());

    // .attach(Logs::init())
    // .mount("/", routes![read]);
    // 处理 错误
    let r = r.register("/", catchers![server_error, bad_request]);
    // 加载 路由
    let r = api::api_routes(r);
    r
}
