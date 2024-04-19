mod user;

/// api 路由
pub fn api_routes(r: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    r.mount("/api", routes![user::login, user::register])
}