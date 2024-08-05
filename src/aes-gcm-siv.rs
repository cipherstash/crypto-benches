use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use aes_gcm_siv::aead::{Aead, KeyInit};
use aes_gcm_siv::{Aes128GcmSiv, Aes256GcmSiv};
use aws_lc_rs::aead::{Aad, Algorithm, BoundKey, NonceSequence, SealingKey, UnboundKey, AES_128_GCM_SIV, AES_256_GCM_SIV};
use aws_lc_rs::error::Unspecified;

const KB: usize = 1024;

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
type Benchmarker = Criterion;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
type Benchmarker = Criterion<criterion_cycles_per_byte::CyclesPerByte>;

struct IntegerNonceSequence(u64);
impl IntegerNonceSequence {
    const fn new() -> Self {
        Self(0)
    }
}

impl NonceSequence for IntegerNonceSequence {
    fn advance(&mut self) -> Result<aws_lc_rs::aead::Nonce, Unspecified> {
        let mut result = [0u8; aws_lc_rs::aead::NONCE_LEN];
        result[4..].copy_from_slice(&u64::to_be_bytes(self.0));
        self.0 = self.0.checked_add(1).ok_or(Unspecified)?;
        Ok(aws_lc_rs::aead::Nonce::assume_unique_for_key(result))
    }
}

fn get_aws_lc_sealing_key(algorithm: &'static Algorithm) -> SealingKey<IntegerNonceSequence> {
    let key_bytes: [u8; 32] = Default::default();
    let len = algorithm.key_len();
    let ukey = UnboundKey::new(algorithm, &key_bytes[..len]).unwrap();
    aws_lc_rs::aead::SealingKey::new(ukey, IntegerNonceSequence::new())
}

fn bench(c: &mut Benchmarker) {
    let mut group = c.benchmark_group("aes-gcm-siv");

    let mut aws_lc_keys = vec![
        (128, get_aws_lc_sealing_key(&AES_128_GCM_SIV)),
        (256, get_aws_lc_sealing_key(&AES_256_GCM_SIV)),
    ];

    for size in &[16, 100, 1000, 2 * KB, 4 * KB] {
        let mut buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(format!("rust-crypto/encrypt-128:{size}"), |b| {
            let cipher = Aes128GcmSiv::new(&Default::default());
            b.iter(|| cipher.encrypt(&Default::default(), &*buf))
        });
        group.bench_function(format!("rust-crypto/decrypt-128:{size}"), |b| {
            let cipher = Aes128GcmSiv::new(&Default::default());
            b.iter(|| cipher.decrypt(&Default::default(), &*buf))
        });

        group.bench_function(format!("rust-crypto/encrypt-256:{size}"), |b| {
            let cipher = Aes256GcmSiv::new(&Default::default());
            b.iter(|| cipher.encrypt(&Default::default(), &*buf))
        });
        group.bench_function(format!("rust-crypto/decrypt-256:{size}"), |b| {
            let cipher = Aes256GcmSiv::new(&Default::default());
            b.iter(|| cipher.decrypt(&Default::default(), &*buf))
        });

        for (keysize, ref mut key) in aws_lc_keys.iter_mut() {
            group.bench_function(format!("aws-lc/encrypt-{keysize}:{size}"), |b| {
                b.iter(|| {
                    let aad = Aad::empty();
                    key.seal_in_place_separate_tag(aad, &mut buf).unwrap()
                })
            });
        }
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
