mod utf8;
mod xor;
mod aes256;


pub use utf8::encode;
pub use xor::{xor_encrypt, generate_key};
pub use aes256::{gf256, sbox};


pub fn encoding(data: &str, use_xor: bool) -> Vec<u8> {
    let bytes = encode(data);
    if use_xor {
        xor_encrypt(&bytes)
    } else {
        bytes
    }
}