use rocket::serde::{json::serde_json, Deserialize, Serialize};
use rocket_sync_db_pools::rusqlite::{params, Connection, Error};
use uuid::Uuid;

use crate::tools::pwd::{pwd_hash, pwd_verify};
use crate::{
    db::SDb,
    tools::{jwt, res::ApiRes, ustr},
};

/// 数据表名
const TABLE_NAME_USER: &str = "user";

/// User 数据库Model
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    /// 主键
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// uuid 自己的唯一ID，用于后续所有的唯一判断
    pub uuid: String,
    /// 账户
    pub username: String,
    /// 密码
    pub password: String,
    /// 创建时间
    // pub create_time: usize,
    /// 昵称
    pub nickname: String,
}

impl User {
    /// 创建的时候
    pub fn new(username: String, password: String) -> Self {
        let hash_pwd = pwd_hash(password).unwrap();
        Self {
            id: None,
            uuid: Uuid::new_v4().to_string(),
            username,
            password: hash_pwd,
            nickname: format!("nickname_{}", ustr::str_len(10)),
        }
    }
    /// 注册用户
    pub async fn register(username: String, password: String, db: &SDb) -> ApiRes {
        db.run(move |conn| {
            // 查找是否有账号
            if User::find_user("username", username.clone(), conn).is_err() {
                return User::create_user(username, password, conn);
            }
            ApiRes::error("".to_string(), "账号已存在，请更改账号".to_string())
        })
        .await
    }
    /// 创建一个用户
    pub fn create_user(username: String, password: String, conn: &mut Connection) -> ApiRes {
        let user = User::new(username, password);
        // 存入数据库
        match conn.execute(
            &format!("INSERT INTO {TABLE_NAME_USER} (username, password,nickname,uuid) VALUES (?1, ?2, ?3, ?4)"),
            params![user.username, user.password, user.nickname, user.uuid],
        ) {
            Ok(_) => ApiRes::success("".to_string(), "注册成功！".to_string()),
            Err(err) => ApiRes::error("".to_string(), format!("创建用户失败！{}", err.to_string())),
        }
    }

    /// ## 查找用户 ##
    /// - 根据 `key` = `value` 查询，可以是 username = "xx"
    /// ```
    /// let u:Result<User, ApiRes> = User::find_user("username", "xx", conn)
    /// ```
    ///- 其中 `ApiRes` 可以直接返回给用户
    pub fn find_user(
        key: &'static str,
        value: String,
        conn: &mut Connection,
    ) -> Result<User, ApiRes> {
        // 查询用户
        match conn.query_row(
            &format!(
                "SELECT id,username,password,nickname,uuid FROM {TABLE_NAME_USER} WHERE {key} = ?1"
            ),
            params![value],
            |f| {
                Ok(User {
                    id: Some(f.get(0)?),
                    username: f.get(1)?,
                    password: f.get(2)?,
                    nickname: f.get(3)?,
                    uuid: f.get(4)?,
                })
            },
        ) {
            Ok(user) => Ok(user),
            Err(e) => match e {
                Error::QueryReturnedNoRows => {
                    Err(ApiRes::error("".to_string(), "用户不存在".to_string()))
                }
                _ => Err(ApiRes::error("".to_string(), "查询错误！".to_string())),
            },
        }
    }
    /// 修改密码
    pub fn changed_pwd(&self, newpwd: String, conn: &mut Connection) -> Result<ApiRes, ApiRes> {
        let pwd = pwd_hash(newpwd)?;
        match conn.execute(
            format!("update {TABLE_NAME_USER} set password = ?1").as_str(),
            params![pwd],
        ) {
            Ok(i) => Ok(ApiRes::success(format!("{i}"), "密码修改成功!".to_string())),
            Err(e) => Err(ApiRes::error(
                e.to_string(),
                "用户密码修改失败！".to_string(),
            )),
        }
    }
    /// 创建表，返回是否创建成功
    pub fn create_table(conn: &mut Connection) -> bool {
        match conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {TABLE_NAME_USER} (
                id INTEGER PRIMARY KEY,
                uuid TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                nickname TEXT NOT NULL
            )"
            ),
            params![],
        ) {
            Ok(_) => {
                println!("User数据表创建成功");
                true
            }
            Err(err) => {
                println!("User数据表创建失败:{}", err.to_string());
                false
            }
        }
    }
    /// 生成jwt
    pub fn get_jwt(&self) -> Result<String, ApiRes> {
        let model = jwt::JwtModel::new(self.uuid.clone(), self.username.clone());
        jwt::get_jwt(model)
    }
    /// 转成 json 字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| err.to_string())
    }
    /// 验证密码是否正确
    pub fn verify_pwd(&self, pwd: String) -> bool {
        pwd_verify(pwd, self.password.clone()).unwrap_or_else(|_| false)
        // match pwd_verify(self.password.clone(), hash) {
        //     Ok(res) => res,
        //     Err(_) => false,
        // }
    }
}
