use ark_std::{rand::Rng, test_rng};
use criterion::{criterion_group, criterion_main, Criterion};
use elliptic_curve::hash_to_curve::{
    optimized_swu::{secp256K1_oswu::Secp256K1OSWUMap, OptimizedSWUMap},
    simplified_swu::{secp256K1_sswu::Secp256K1SSWUMap, SimplifiedSWUMap},
    sw_map::{secp256K1_sw::Secp256K1SWMap, SWMap},
};
use sha2::Sha256;

fn bench_secp256k1_sw(c: &mut Criterion) {
    let mut rng = test_rng();
    let mut group = c.benchmark_group("secp256k1_sw");
    group.bench_function("sw".to_string(), |b| {
        b.iter(|| {
            let msg: Vec<u8> = (0..100).map(|_| rng.gen()).collect();
            _ = Secp256K1SWMap::hash::<Sha256>(&msg);
        });
    });
    group.finish();
}

fn bench_secp256k1_sswu(c: &mut Criterion) {
    let mut rng = test_rng();
    let mut group = c.benchmark_group("secp256k1_sswu");
    group.bench_function("sswu".to_string(), |b| {
        b.iter(|| {
            let msg: Vec<u8> = (0..100).map(|_| rng.gen()).collect();
            _ = Secp256K1SSWUMap::hash::<Sha256>(&msg);
        });
    });
    group.finish();
}

fn bench_secp256k1_oswu(c: &mut Criterion) {
    let mut rng = test_rng();
    let mut group = c.benchmark_group("secp256k1_oswu");
    group.bench_function("oswu".to_string(), |b| {
        b.iter(|| {
            let msg: Vec<u8> = (0..100).map(|_| rng.gen()).collect();
            _ = Secp256K1OSWUMap::hash::<Sha256>(&msg);
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_secp256k1_sw,
    bench_secp256k1_sswu,
    bench_secp256k1_oswu
);
criterion_main!(benches);
