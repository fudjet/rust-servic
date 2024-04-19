pub mod models;

use rocket::{fairing::AdHoc, Build, Rocket};

use rocket_sync_db_pools::{
    database,
    rusqlite::{params, Connection},
};

#[database("rusqlite")]
pub struct SDb(Connection);

// 初始化表格
async fn init_db(r: Rocket<Build>) -> Rocket<Build> {
    SDb::get_one(&r)
        .await
        .expect("数据库链接。。")
        .run(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS user (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL
                )",
                params![],
            )
            .expect("数据库创建失败");
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
