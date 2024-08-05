use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use sha2::{Sha256, Digest};
use sha3::{Sha3_256, Shake128, Shake256, TurboShake256, TurboShake128, TurboShake128Core, TurboShake256Core};
use sha3::digest::{ExtendableOutput, Update};
use tiny_keccak::{KangarooTwelve, Xof, IntoXof, Hasher};

const KB: usize = 1024;
type Benchmarker = Criterion;

fn bench(c: &mut Benchmarker) {
    let mut group = c.benchmark_group("hashes");

    for size in &[10, 64, 128, 512, KB, 2 * KB, 4 * KB] {
        let buf = vec![0u8; *size];
        let mut out = [0u8; 32];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("rust-crypto-sha2-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                Sha256::digest(input);
            })
        });
        group.bench_with_input(BenchmarkId::new("ring-sha2-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                ring::digest::digest(&ring::digest::SHA256, input)
            })
        });
        group.bench_with_input(BenchmarkId::new("rust-crypto-sha3-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                Sha3_256::digest(&input)
            })
        });
        group.bench_with_input(BenchmarkId::new("rust-crypto-shake-128", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { Shake128::digest_xof(input, &mut out) })
        });
        group.bench_with_input(BenchmarkId::new("rust-crypto-shake-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| Shake256::digest_xof(input, &mut out))
        });
        // TODO: Create a 256 group
        group.bench_with_input(BenchmarkId::new("rust-crypto-turboshake-128", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                let mut x = TurboShake128::from_core(TurboShake128Core::new(1));
                x.update(input);
                x.finalize_xof_into(&mut out);
            })
        });
        group.bench_with_input(BenchmarkId::new("rust-crypto-turboshake-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                let mut x = TurboShake256::from_core(TurboShake256Core::new(1));
                x.update(input);
                x.finalize_xof_into(&mut out);
            })
        });
        group.bench_with_input(BenchmarkId::new("tiny-keccak-kangaroo-12", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| {
                let mut hasher = KangarooTwelve::new(b"");
                hasher.update(input);
                let mut xof = hasher.into_xof();
                xof.squeeze(&mut out[..32]);
            })
        });
        group.bench_with_input(BenchmarkId::new("blake3-256", size), buf.as_ref(), |b, input: &[u8]| {
            b.iter(|| { blake3::hash(input); })
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
