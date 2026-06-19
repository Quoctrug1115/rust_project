use sha2::{Digest, Sha256};

pub fn hash_block(data: &[u8]) -> [u8;32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&result);
    hash_array
}
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
fn main() {
    let data1 = b"Hello, world!".to_vec();
    let mut data2 = data1.clone();
    data2[0] ^= 1;
    let hash1 = hash_block(&data1);
    let hash2 = hash_block(&data2);
    let mut diffs_bits = 0;
    for i in 0..32 {
        let diff_byte = hash1[i] ^ hash2[i];
        diffs_bits += diff_byte.count_ones();
    }
    println!("Data 1 (String): {:?}", String::from_utf8_lossy(&data1));
    println!("Data 2 (String): {:?}", String::from_utf8_lossy(&data2));

    println!("Hash 1: {}", bytes_to_hex_string(&hash1));
    println!("Hash 2: {}", bytes_to_hex_string(&hash2));
    
    println!("Tong so bit khac nhau : {}", diffs_bits);

    let percentage = (diffs_bits as f64 / 256.0) * 100.0;
    println!("Ti le thay doi: {:.2}%", percentage);
    
    if percentage >45.0 && percentage < 55.0 {
        println!("xoay quanh 50%");
    }
}