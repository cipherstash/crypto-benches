use rand::{thread_rng, RngCore};
use ring::{aead::{self, UnboundKey, BoundKey, NonceSequence, AES_256_GCM, Nonce, Aad}, error::Unspecified};

pub const NONCE: [u8; 96 / 8] = [0u8; 96 / 8];

struct IntegerNonceSequence(u64);
impl IntegerNonceSequence {
    const fn new() -> Self {
        Self(0)
    }
}

impl NonceSequence for IntegerNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut result = [0u8; aead::NONCE_LEN];
        result[4..].copy_from_slice(&u64::to_be_bytes(self.0));
        self.0 = self.0.checked_add(1).ok_or(Unspecified)?;
        Ok(aead::Nonce::assume_unique_for_key(result))
    }
}

fn main() {
    let mut key_bytes: [u8; 32] = Default::default();
    let mut rng = thread_rng();
    rng.try_fill_bytes(&mut key_bytes).unwrap();
    let ukey = UnboundKey::new(&AES_256_GCM, &key_bytes).unwrap();
    let mut key = ring::aead::SealingKey::new(ukey, IntegerNonceSequence::new());
    let aad = Aad::from(b"myaad");

    let mut in_out = vec![0u8; 1024 + AES_256_GCM.tag_len()];
    let tag = key.seal_in_place_separate_tag(aad, &mut in_out).unwrap();

    dbg!(tag.as_ref().len());
    dbg!(in_out.len());
}
