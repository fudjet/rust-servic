use rocket::serde::{Deserialize, json::serde_json, Serialize};

/// 统一返回代码

pub enum ApiResCode {
    /// 请求成功
    Success,
    /// 请求失败
    Error,
    /// 未登入
    NotLogin,
}

/// 统一返回类型
// pub type ApiResType = Json<ApiRes>;
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiRes {
    pub code: i64,
    pub data: String,
    pub message: String,
}

impl ApiRes {
    /// 新建
    pub fn new(code: ApiResCode, data: String, message: String) -> Self {
        let code = match code {
            ApiResCode::Success => 0,
            ApiResCode::Error => 1,
            ApiResCode::NotLogin => -1,
        };
        Self {
            code,
            data,
            message,
        }
    }
    /// 成功
    pub fn success(data: String, message: String) -> Self {
        Self::new(ApiResCode::Success, data, message.to_string())
    }
    /// 失败
    pub fn error(data: String, message: String) -> Self {
        Self::new(ApiResCode::Error, data, message)
    }
    /// 未登入
    pub fn not_login(data: String, message: String) -> Self {
        Self::new(ApiResCode::NotLogin, data, message.to_string())
    }
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| String::from(""))
    }
}
