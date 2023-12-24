use aes_gcm::aes::{Aes256, Aes128};
use cmac::{Cmac, Mac};



fn main() {
    let key: [u8; 32] = Default::default();
    let input: [u8; 10] = Default::default();
    let m = Cmac::<Aes128>::new_from_slice(&key[..16]).unwrap().chain_update(input).finalize();
}
