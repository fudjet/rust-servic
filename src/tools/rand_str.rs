use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// 随机字符串
pub fn str_len(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
