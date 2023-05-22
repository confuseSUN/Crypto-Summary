use ark_std::{rand::Rng, test_rng};
use criterion::{criterion_group, criterion_main, Criterion};
use elliptic_curve::hash_to_curve::{
    secp256K1_sswu::Secp256K1SSWUMap, secp256K1_sw::Secp256K1SWMap,
    simplified_swu::SimplifiedSWUMap, sw_map::SWMap,
};
use sha2::Sha256;

fn bench_secp256k1_sswu(c: &mut Criterion) {
    let mut group = c.benchmark_group("secp256k1_sswu");
    group.bench_function("sswu".to_string(), |b| {
        b.iter(|| {
            let mut rng = test_rng();
            let msg: Vec<u8> = (0..100).map(|_| rng.gen()).collect();
            let point = Secp256K1SSWUMap::hash::<Sha256>(&msg).unwrap();
            assert!(point.is_on_curve())
        });
    });
    group.finish();
}

fn bench_secp256k1_sw(c: &mut Criterion) {
    let mut group = c.benchmark_group("secp256k1_sw");
    group.bench_function("sw".to_string(), |b| {
        b.iter(|| {
            let mut rng = test_rng();
            let msg: Vec<u8> = (0..100).map(|_| rng.gen()).collect();

            let point = Secp256K1SWMap::hash::<Sha256>(&msg).unwrap();
            assert!(point.is_on_curve())
        });
    });
    group.finish();
}

criterion_group!(benches, bench_secp256k1_sswu, bench_secp256k1_sw);
criterion_main!(benches);
