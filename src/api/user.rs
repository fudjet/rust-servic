use crate::db::{
    models::{ApiRes, User},
    SDb,
};
use rocket::serde::{
    json::{Error, Json},
    Deserialize,
};
use rocket_sync_db_pools::rusqlite::params;
// 登入接口
#[post("/login")]
pub fn login() -> String {
    "".to_string()
}

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RegisterModel {
    /// 用户名
    username: String,
    /// 密码
    password: String,
}

/// 注册和登入
///
/// 如果用户存在切密码正正确，返回token
///
/// 如果用户存在切密码错误，返回错误
///
/// 如果用户不存在，直接注册账号
// #[post("/register", data = "<task>")]
// pub async fn register(db: SDb, task: Result<Json<RegisterModel>, Error<'_>>) -> Json<ApiRes> {
//     if let Ok(json) = task {
//         let usn = json.username.clone();
//         // 查询用户
//         let post = db
//             .run(move |conn| {
//                 conn.query_row(
//                     "SELECT id FROM user WHERE username = ?1",
//                     params![usn],
//                     |r| {
//                         Ok(User {
//                             id: r.get(0)?,
//                             username: r.get(1)?,
//                             password: r.get(2)?,
//                         })
//                     },
//                 )
//             })
//             .await;
//         // 用户存不存在
//         if let Err(e) = post {
//             if e.to_string() == "Query returned no rows" {
//                 // 开始注册
//                 if let Ok(_) = db
//                     .run(move |conn| {
//                         conn.execute(
//                             "INSERT INTO user (username, password) VALUES (?1, ?2)",
//                             params![json.username, json.password],
//                         )
//                     })
//                     .await
//                 {
//                     return ApiRes::success("".to_string(), "注册成功");
//                 }
//                 return ApiRes::error("注册失败!");
//             }
//             // 其他错误
//             return ApiRes::error(e.to_string().as_str());
//         }
//         return ApiRes::error("客户已存在!");
//     }
//     ApiRes::error("数据错误哦")
// }
#[post("/register", data = "<task>")]
pub async fn register(db: SDb, task: Result<Json<RegisterModel>, Error<'_>>) -> Json<ApiRes> {
    match task {
        Ok(json) => {
            let username = json.username.clone();
            // Attempt to find existing user
            let user_exists = db
                .run(move |conn| {
                    conn.query_row(
                        "SELECT EXISTS(SELECT 1 FROM user WHERE username = ?1)",
                        params![username],
                        |r| r.get(0),
                    )
                })
                .await
                .unwrap_or(false); // Default to false if query fails
            if !user_exists {
                // Proceed with registration if user does not exist
                let registration_result = db
                    .run(move |conn| {
                        conn.execute(
                            "INSERT INTO user (username, password) VALUES (?1, ?2)",
                            params![&json.username, &json.password],
                        )
                    })
                    .await;
                match registration_result {
                    Ok(_) => ApiRes::success("".to_string(), "注册成功"),
                    Err(_) => ApiRes::error("注册失败!"),
                }
            } else {
                // User already exists
                ApiRes::error("客户已存在!")
            }
        }
        Err(_) => ApiRes::error("数据错误哦"),
    }
}
