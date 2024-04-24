use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::res::ApiRes;

const JWT_KEY: &str = "secret";

/// jwt 模型
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtModel {
    /// UUID
    uuid: String,
    /// 账户
    username: String,
    /// 验证时间戳
    exp: usize,
}

impl JwtModel {
    /// 创建 jwtmodel
    ///
    /// 默认过期为 7天
    pub fn new(uuid: String, username: String) -> Self {
        Self::new_days(uuid, username, 7)
    }
    /// 创建 jwtmodel
    ///
    /// days 为过期时间，单位为天
    pub fn new_days(uuid: String, username: String, days: usize) -> Self {
        // 获取当前时间戳
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            uuid,
            username,
            exp: now + days * 24 * 3600,
        }
    }
}

/// 获取 jwt 根据jwt model
pub fn get_jwt(model: JwtModel) -> Result<String, ApiRes> {
    // 创建 header 编码
    // let mut header = Header::new(Algorithm::HS512);
    // 设置 kid
    // header.kid = Some(JWT_KEY.to_owned());
    // 设置 密钥
    let key = EncodingKey::from_secret(JWT_KEY.as_ref());
    // 编码
    if let Ok(token) = encode(&Header::default(), &model, &key) {
        return Ok(token);
    }
    // 失败后返回空
    Err(ApiRes::error("".to_string(), "token 生成失败".to_string()))
}

/// 解码jwt
pub fn decode_jwt(token: String) -> Result<JwtModel, ApiRes> {
    // 创建解码密钥
    let key = DecodingKey::from_secret(JWT_KEY.as_ref());
    // 解码
    if let Ok(token) = decode::<JwtModel>(&token, &key, &Validation::new(Algorithm::HS512)) {
        return Ok(token.claims);
    }
    // 失败后返回空
    Err(ApiRes::error("".to_string(), "token 解码失败".to_string()))
}
