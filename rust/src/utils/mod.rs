use rand::Rng;

fn byte_to_hex(byte: &u8) -> String {
    format!("{byte:02x}")
}

/// Serializes bytes into a hex string
pub fn to_hex_string<T: Clone + Into<Vec<u8>>>(bytes: &T) -> String {
    let hex_vec: Vec<String> = bytes.clone().into().iter().map(byte_to_hex).collect();

    hex_vec.join("")
}

/* 随机访问元素 */
pub fn random_access<T: Clone>(nums: &[T]) -> T {
    // 在区间 [0, nums.len()) 中随机抽取一个数字
    let random_index = rand::thread_rng().gen_range(0..nums.len());
    // 获取并返回随机元素
    let random_num = &nums[random_index];
    random_num.clone()
}
