use rocket::serde::{json::serde_json, Deserialize, Serialize};

/// 统一返回代码
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
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
    pub code: ApiResCode,
    pub data: String,
    pub message: String,
}
impl ApiRes {
    /// 新建
    pub fn new(code: ApiResCode, data: &'static str, message: String) -> Self {
        Self {
            code,
            data: data.to_string(),
            message,
        }
    }
    /// 成功
    pub fn success(data: &'static str, message: &'static str) -> Self {
        Self::new(ApiResCode::Success, data, message.to_string())
    }
    /// 失败
    pub fn error(data: &'static str, message: String) -> Self {
        Self::new(ApiResCode::Error, data, message)
    }
    /// 未登入
    pub fn not_login(data: &'static str, message: &'static str) -> Self {
        Self::new(ApiResCode::NotLogin, data, message.to_string())
    }
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match serde_json::to_string(self) {
            Ok(s) => s,
            Err(_) => String::from(""),
        }
    }
}
