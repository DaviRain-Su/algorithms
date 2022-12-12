fn byte_to_hex(byte: &u8) -> String {
    format!("{byte:02x}")
}

/// Serializes bytes into a hex string
pub fn to_hex_string<T: Clone + Into<Vec<u8>>>(bytes: &T) -> String {
    let hex_vec: Vec<String> = bytes.clone().into().iter().map(byte_to_hex).collect();

    hex_vec.join("")
}
