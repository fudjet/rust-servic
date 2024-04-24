use rocket::{
    serde::{
        json::{Error, Json},
        Deserialize,
    },
    Route,
};

use crate::{
    db::{user::User, SDb},
    tools::res::ApiRes,
};

/// 注册用的model
#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RegisterModel {
    /// 用户名
    username: String,
    /// 密码
    password: String,
}

// ==================================================================================
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
                        if user.verify_pwd(json.password.clone()) {
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

// ==================================================================================
/// 注册
#[post("/register", data = "<body>")]
pub async fn register(db: SDb, body: Result<Json<RegisterModel>, Error<'_>>) -> String {
    match body {
        Ok(json) => User::register(json.username.clone(), json.password.clone(), &db)
            .await
            .to_string(),
        Err(e) => ApiRes::error("".to_string(), format!("{}", e)).to_string(),
    }
}

// ==================================================================================
/// 修改密码用的model
#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ChangedModel {
    /// 用户名
    username: String,
    /// 旧密码
    old: String,
    /// 新密码
    new: String,
}

/// 修改密码
#[post("/changed_pwd", data = "<body>")]
pub async fn changed_pwd(db: SDb, body: Result<Json<ChangedModel>, Error<'_>>) -> String {
    match body {
        Ok(json) => {
            // 读取用户，和判断密码正确与否
            db.run(
                move |conn| match User::find_user("username", json.username.clone(), conn) {
                    Ok(user) => {
                        // 判断密码是否都一样
                        if json.old == json.new {
                            return ApiRes::error(
                                "".to_string(),
                                "新密码与旧密码相同！".to_string(),
                            );
                        }
                        // 判断密码是否正确
                        if user.verify_pwd(json.old.clone()) {
                            // 修改密码
                            return user
                                .changed_pwd(json.new.clone(), conn)
                                .unwrap_or_else(|e| e);
                        }
                        ApiRes::error("".to_string(), "密码错误".to_string())
                    }
                    Err(e) => e,
                },
            )
            .await
            .to_string()
        }
        Err(e) => ApiRes::error(e.to_string(), "请传入完整数据！".to_string()).to_string(),
    }
}

/// 获取路由列表
pub fn get_api_list() -> Vec<Route> {
    routes![login, register, changed_pwd]
}
