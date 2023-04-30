use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use rand::{thread_rng, RngCore};
use ring::{aead::{self, UnboundKey, BoundKey, NonceSequence, AES_256_GCM, Nonce, Aad}, error::Unspecified};

const KB: usize = 1024;

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
type Benchmarker = Criterion;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
type Benchmarker = Criterion<criterion_cycles_per_byte::CyclesPerByte>;

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

fn bench(c: &mut Benchmarker) {
    let mut group = c.benchmark_group("aes-gcm");

    for size in &[KB, 2 * KB, 4 * KB] {
        let mut buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(format!("rust-crypto/encrypt-128:{}", size), |b| {
            let cipher = Aes128Gcm::new(&Default::default());
            b.iter(|| cipher.encrypt(&Default::default(), &*buf))
        });
        group.bench_function(format!("rust-crypto/decrypt-128:{}", size), |b| {
            let cipher = Aes128Gcm::new(&Default::default());
            b.iter(|| cipher.decrypt(&Default::default(), &*buf))
        });

        group.bench_function(format!("rust-crypto/encrypt-256:{}", size), |b| {
            let cipher = Aes256Gcm::new(&Default::default());
            b.iter(|| cipher.encrypt(&Default::default(), &*buf))
        });
        group.bench_function(format!("rust-crypto/decrypt-256:{}", size), |b| {
            let cipher = Aes256Gcm::new(&Default::default());
            b.iter(|| cipher.decrypt(&Default::default(), &*buf))
        });

        group.bench_function(format!("ring/encrypt-256:{}", size), |b| {
            let mut key_bytes: [u8; 32] = Default::default();
            let mut rng = thread_rng();
            rng.try_fill_bytes(&mut key_bytes).unwrap();
            let ukey = UnboundKey::new(&AES_256_GCM, &key_bytes).unwrap();
            let mut key = ring::aead::SealingKey::new(ukey, IntegerNonceSequence::new());
            b.iter(|| {
                let aad = Aad::from(b"myaad");
                key.seal_in_place_separate_tag(aad, &mut buf).unwrap()
            })
        });
    }

    group.finish();
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(criterion_cycles_per_byte::CyclesPerByte);
    targets = bench
);

criterion_main!(benches);
