mod aes256;
mod gf256;
mod sbox;
mod utf8;
mod xor;

pub use aes256::{core, key_expansion};
pub use gf256::GF256;
pub use sbox::SBox;
pub use utf8::encode;
pub use xor::{generate_key, xor_encrypt};

pub fn encoding(data: &str, use_xor: bool) -> Vec<u8> {
    let bytes = encode(data);
    if use_xor { xor_encrypt(&bytes) } else { bytes }
}
