use anyhow::Result;
use rocket::serde::{
    json::{Error, Json},
    Deserialize,
};

use crate::tools::pwd::pwd_verify;
use crate::{
    db::{user::User, SDb},
    tools::res::ApiRes,
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
#[post("/login", data = "<body>")]
pub async fn login(db: SDb, body: Result<Json<RegisterModel>, Error<'_>>) -> String {
    match body {
        Ok(json) => {
            // 查询用户
            db.run(move |conn| {
                match User::find_user("username", json.username.clone(), conn) {
                    Ok(user) => {
                        // 验证密码
                        if pwd_verify(json.password.clone(), user.password.clone()).is_ok() {
                            return match user.get_jwt() {
                                // 生成 token 返回
                                Ok(jwt) => ApiRes::success(jwt, "登入成功".to_string()),
                                Err(e) => e,
                            };
                        }
                        ApiRes::error("".to_string(), "密码错误".to_string())
                    }
                    Err(e) => e,
                }
            })
            .await
            .to_string()
        }
        Err(e) => ApiRes::error("".to_string(), format!("{}", e)).to_string(),
    }
}

/// 注册
#[post("/register", data = "<task>")]
pub async fn register(db: SDb, task: Result<Json<RegisterModel>, Error<'_>>) -> String {
    match task {
        Ok(json) => User::register(json.username.clone(), json.password.clone(), &db)
            .await
            .to_string(),
        Err(e) => ApiRes::error("".to_string(), format!("{}", e)).to_string(),
    }
}
