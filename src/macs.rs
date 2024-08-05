use aes::{Aes128, Aes256};
use cmac::Cmac;
use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use hmac::{Hmac, Mac};
use pmac::Pmac;
use sha2::Sha256;

const KB: usize = 1024;
type Benchmarker = Criterion;

fn bench(c: &mut Benchmarker) {
    let key: [u8; 32] = Default::default();
    let mut group = c.benchmark_group("macs");

    for size in &[10, 64, 128, 512, KB, 2 * KB, 4 * KB] {
        let buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("rust-crypto-hmac-sha2", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                Hmac::<Sha256>::new_from_slice(&key).unwrap().chain_update(input).finalize();
            })
        });
        group.bench_with_input(BenchmarkId::new("blake3", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { blake3::keyed_hash(&key, input); })
        });
        group.bench_with_input(BenchmarkId::new("cmac-aes-128", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { Cmac::<Aes128>::new_from_slice(&key[..16]).unwrap().chain_update(input).finalize(); })
        });
        group.bench_with_input(BenchmarkId::new("cmac-aes-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { Cmac::<Aes256>::new_from_slice(&key).unwrap().chain_update(input).finalize(); })
        });
        group.bench_with_input(BenchmarkId::new("pmac-aes-128", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { Pmac::<Aes128>::new_from_slice(&key[..16]).unwrap().chain_update(input).finalize(); })
        });
        group.bench_with_input(BenchmarkId::new("pmac-aes-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { Pmac::<Aes256>::new_from_slice(&key).unwrap().chain_update(input).finalize(); })
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);

criterion_main!(benches);
