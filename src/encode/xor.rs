use crate::utils::write_to_file;
use rand::RngCore;
use rand::rngs::OsRng;

pub fn generate_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    let mut rng = OsRng; // Создаём экземпляр
    rng.fill_bytes(&mut key);
    write_to_file(&key, "xor_key");
    key
}

pub fn xor_encrypt(data: &[u8]) -> Vec<u8> {
    let key = &generate_key(data.len());
    let resp: Vec<u8> = data
        .iter()
        .zip(key.iter().cycle())
        .map(|(&a, &b)| a ^ b)
        .collect();
    write_to_file(&resp, "xor_encrypt");
    resp
}
