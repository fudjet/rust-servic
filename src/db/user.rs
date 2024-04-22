use crate::{
    db::SDb,
    tools::{rand_str, res::ApiRes},
};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::rusqlite::{params, Connection, Error};
use uuid::Uuid;

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
    /// 昵称
    pub nickname: String,
}
impl User {
    /// 创建的时候
    pub fn create(username: String, password: String) -> Self {
        Self {
            id: None,
            uuid: Uuid::new_v4().to_string(),
            username,
            password,
            nickname: format!("Usernick_{}", rand_str::str_len(10)),
        }
    }
    /// 注册用户
    pub async fn register(username: String, password: String, db: &SDb) -> ApiRes {
        db.run(move |conn| {
            // 查找是否有账号
            if !User::have_user("username", username.clone(), conn) {
                return User::create_user(username, password, conn);
            }
            ApiRes::error("", "账号已存在，请更改账号".to_string())
        })
        .await
    }
    /// 创建一个用户
    pub fn create_user(username: String, password: String, conn: &mut Connection) -> ApiRes {
        let user = User::create(username, password);
        // 存入数据库
        match conn.execute(
            "INSERT INTO user (username, password,nickname,uuid) VALUES (?1, ?2, ?3, ?4)",
            params![user.username, user.password, user.nickname, user.uuid],
        ) {
            Ok(_) => ApiRes::success("", "注册成功！"),
            Err(err) => ApiRes::error("", format!("创建用户失败！{}", err.to_string())),
        }
    }
    /// 是否有这个用户
    ///
    /// - true 有这个用户
    ///
    /// - false 没有这个用户
    pub fn have_user(key: &'static str, value: String, conn: &mut Connection) -> bool {
        let result: Result<i64, Error> = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM ?1 WHERE ?2 = ?3)",
            params![TABLE_NAME_USER, key, value],
            |r| r.get(0),
        );
        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    /// 查找用户
    // pub async fn find_user_name(key: &'static str, value: String, db: &SDb) {}
    /// 创建表，返回是否创建成功
    pub fn create_table(conn: &mut Connection) -> bool {
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY,
                uuid TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                nickname TEXT NOT NULL
            )",
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
}
