pub mod client;
pub mod user;
// web router
use rocket::{fairing::AdHoc, Build, Rocket};

use rocket_sync_db_pools::{database, rusqlite::Connection};

use self::user::User;

#[database("rusqlite")]
pub struct SDb(Connection);

// 初始化表格
async fn init_db(r: Rocket<Build>) -> Rocket<Build> {
    SDb::get_one(&r)
        .await
        .expect("数据库链接。。")
        .run(|conn: &mut Connection| {
            // 创建
            User::create_table(conn);
        })
        .await;
    r
}
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("rusqlite", |r| async {
        r.attach(SDb::fairing())
            .attach(AdHoc::on_ignite("rusqlite init", init_db))
    })
}
