use rocket::serde::json::serde_json;
use rocket::serde::{json::Json, Deserialize, Serialize};
/// 初始组件
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BesaModel {
    /** 主键 */
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /** 账户 */
    pub username: String,
    /** 密码 */
    pub password: String,
}
impl BesaModel {
    /// 反序列化
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    /** 主键 */
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /** 账户 */
    pub username: String,
    /** 密码 */
    pub password: String,
}

/// api响应模型
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiRes {
    pub code: i8,
    /** 账户 */
    pub data: String,
    /** 密码 */
    pub msg: String,
}
impl ApiRes {
    /// 成功响应
    pub fn success(data: String, msg: &str) -> Json<Self> {
        Json(ApiRes {
            code: 0,
            data,
            msg: msg.to_string(),
        })
    }
    /// 错误响应
    pub fn error(msg: &str) -> Json<Self> {
        Json(ApiRes {
            code: -1,
            data: Default::default(),
            msg: msg.to_string(),
        })
    }
}
