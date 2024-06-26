use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};

pub fn read_file(filecontents: &Vec<u8>) -> &[u8] {
    let to_return = &filecontents.as_slice();
    to_return
}
pub fn string_to_bytes(input: &str) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let input_bytes = input.as_bytes();

    for (i, &byte) in input_bytes.iter().enumerate() {
        if i < 32 {
            bytes[i] = byte;
        } else {
            break;
        }
    }

    bytes
}

pub fn encrypt_file(key: &[u8], nonce: &[u8], filename: &str) -> Vec<u8> {
    let filecontents = std::fs::read(filename).unwrap();
    let plaintext = read_file(&filecontents);
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
    cipher.encrypt(nonce, plaintext).unwrap()
}

pub fn decrypt_bytes(key: &[u8], nonce: &[u8], bytes: Vec<u8>) -> Vec<u8> {
    let plaintext = bytes.as_slice();
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message

    cipher.decrypt(nonce, plaintext).expect("Banana")
}
