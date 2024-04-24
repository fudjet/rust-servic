// 密码工具

use super::res::ApiRes;

const PWD_SALT: &str = "rust-servic";

/// 密码编码
pub fn pwd_hash(pwd: String) -> Result<String, ApiRes> {
    match bcrypt::hash(pwd_add_salt(pwd), bcrypt::DEFAULT_COST) {
        Ok(hash) => Ok(hash),
        Err(e) => Err(ApiRes::error(
            e.to_string(),
            "密码hash失败。请联系管理员".to_string(),
        )),
    }
}

/// 密码进行加盐处理
fn pwd_add_salt(pwd: String) -> String {
    format!("{PWD_SALT}{pwd}")
}

/// 进行密码校验
pub fn pwd_verify(pwd: String, hash: String) -> Result<bool, ApiRes> {
    match bcrypt::verify(pwd_add_salt(pwd), &hash) {
        Ok(res) => Ok(res),
        Err(e) => Err(ApiRes::error(
            e.to_string(),
            "密码校验失败。请联系管理员".to_string(),
        )),
    }
}
