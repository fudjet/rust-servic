use base64::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
/// 随机字符串
pub fn str_len(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// 当前时间戳转64进制
pub fn time_to_str() -> String {
    // 获取当前时间
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    int_to_str(since_epoch as usize)
}

/// 正整数转64进制
pub fn int_to_str(num: usize) -> String {
    // 获取当前时间
    let since_epoch = num;
    // 64进制
    BASE64_URL_SAFE.encode(since_epoch.to_string())
}
