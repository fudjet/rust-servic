use crate::{
    db::{user::User, SDb},
    tools::res::ApiRes,
};
use anyhow::Result;
use rocket::serde::{
    json::{Error, Json},
    Deserialize,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RegisterModel {
    /// 用户名
    username: String,
    /// 密码
    password: String,
}
/// 登入接口
///
/// 如果用户存在切密码正确，返回token
///
///  data = "<task>"
/// db: SDb, task: Result<Json<RegisterModel>, Error>
///
/// 否则 报错
#[post("/login")]
pub fn login() -> String {
    "123".to_string()
}
/// 注册
///
/// 如果用户存在切密码正正确，返回token
///
/// 如果用户存在切密码错误，返回错误
///
/// 如果用户不存在，直接注册账号
#[post("/register", data = "<task>")]
pub async fn register(db: SDb, task: Result<Json<RegisterModel>, Error<'_>>) -> String {
    match task {
        Ok(json) => User::register(json.username.clone(), json.password.clone(), &db)
            .await
            .to_string(),
        Err(e) => ApiRes::error("", format!("{}", e)).to_string(),
    }
}
// pub async fn register(db: SDb, task: Result<Json<RegisterModel>, Error<'_>>) -> Json<ApiRes> {
//     match task {
//         Ok(json) => {
//             let username = json.username.clone();
//             // Attempt to find existing user
//             let user_exists = db
//                 .run(move |conn| {
//                     conn.query_row(
//                         "SELECT EXISTS(SELECT 1 FROM user WHERE username = ?1)",
//                         params![username],
//                         |r| r.get(0),
//                     )
//                 })
//                 .await
//                 .unwrap_or(false); // Default to false if query fails
//             if !user_exists {
//                 // Proceed with registration if user does not exist
//                 let registration_result = db
//                     .run(move |conn| {
//                         conn.execute(
//                             "INSERT INTO user (username, password) VALUES (?1, ?2)",
//                             params![&json.username, &json.password],
//                         )
//                     })
//                     .await;
//                 match registration_result {
//                     Ok(_) => ApiRes::success("".to_string(), "注册成功"),
//                     Err(_) => ApiRes::error("注册失败!"),
//                 }
//             } else {
//                 // User already exists
//                 ApiRes::error("客户已存在!")
//             }
//         }
//         Err(_) => ApiRes::error("数据错误哦"),
//     }
// }
