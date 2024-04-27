use regex::Regex;
use rocket::Data;
// use rocket::tokio::{fs::File, io::AsyncWriteExt};

use crate::tools::ustr;
use std::{fs, io::Write};
/*
 文件管理，所有文件都会自定义文件名称
 文件定义规则
*/

/// 文件存储位置
const FILE_DIR: &str = "static";

/// 获取新的文件地址
pub fn get_new_path(name: &'static str) -> String {
    init_path();
    format!("{FILE_DIR}/{}_{name}", ustr::str_len(10))
}
/// 初始化存储目录
fn init_path() {
    // 查询是否有指定目录
    if fs::read_dir(FILE_DIR).is_err() {
        match fs::create_dir(FILE_DIR) {
            Ok(_) => println!("创建成功存储目录"),
            Err(e) => println!("创建失败存储目录:{}", e),
        }
    }
}

/// 获取文件存储目录
pub fn get_dir() -> &'static str {
    init_path();
    FILE_DIR
}

///  form上传文件处理 data的二进制
///
pub struct FormFile {
    /// 文件名字 xx.jpg
    pub name: String,
    /// 文件类型 image/jpeg
    pub file_type: String,
    /// 文件大小(B)
    pub size: usize,
    /// 文件的二进制本体
    pub bytes: Vec<u8>,
    /// 文件保存路径
    pub path: String,
}
impl FormFile {
    /// 解析文件
    pub fn new(data: Vec<u8>) -> Self {
        let (x, y) = get_point(&data);
        // 获取 文件本体
        let file_bytes = &data[x..y];
        let (name, file_type) = get_name_type(&data[0..(x - 1)]);
        Self {
            name: name.clone(),
            file_type,
            size: file_bytes.len(),
            bytes: file_bytes.to_vec(),
            path: format!("{FILE_DIR}/{}_{}", ustr::str_len(10), name.clone()),
        }
    }
    /// 保存文件到某地
    pub fn save_path(&self) -> Result<(), String> {
        match fs::File::create(self.path.clone()) {
            Ok(mut f) => match f.write(self.bytes.as_slice()) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e.to_string()),
            },
            Err(e) => return Err(e.to_string()),
        }
    }
    /// 新建切保存
    pub fn new_save(data: Vec<u8>) -> Result<Self, String> {
        let ff = Self::new(data);
        match ff.save_path() {
            Ok(_) => Ok(ff),
            Err(e) => Err(e),
        }
    }
}

/// 获取文件名
fn get_name_type(b: &[u8]) -> (String, String) {
    let text = String::from_utf8_lossy(b).to_string();

    let re = Regex::new(r#"filename="([^"]*)"#).unwrap();
    let filename = re
        .captures(&text)
        .and_then(|cap| cap.get(1))
        .map_or("", |m| m.as_str());

    let re = Regex::new(r#"Content-Type: ([^\n]*)"#).unwrap();
    let content_type = re
        .captures(&text)
        .and_then(|cap| cap.get(1))
        .map_or("", |m| m.as_str());

    // println!("name: {}", content_type); // Prints: Content-Type: image/png
    // println!("filename: {}", filename); // Prints: filename: code1.png
    (filename.to_string(), content_type.to_string())
}

/// 获取锚点，返回 x,y 如果都为0，则表示没有
fn get_point(b: &Vec<u8>) -> (usize, usize) {
    let mut top: usize = 0;
    // 查找顶部锚点
    for i in 0..b.len() {
        // 防止溢出
        if i + 4 > b.len() {
            // 返回错误
            break;
        }
        // 13,10,13,10   "\r\n\r\n"
        if b[i] == 13 && b[i + 1] == 10 && b[i + 2] == 13 && b[i + 3] == 10 {
            top = i + 4;
            break;
        }
    }
    // 获取 底部标点
    let mut bt: usize = 0;
    for i in 0..b.len() {
        let i = b.len() - i;
        if i < 3 {
            break;
        }
        // 13, 10, 45   "\r\n-"
        if b[i - 1] == 45 && b[i - 2] == 10 && b[i - 3] == 13 {
            bt = i - 4;
            break;
        }
    }
    (top, bt)
}
