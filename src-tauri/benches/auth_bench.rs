//! Benchmarks for authentication functions (password hashing)
//!
//! Run with: cargo bench --bench auth_bench

use brisas_app_lib::services::auth::{hash_password, verify_password};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_hash_password(c: &mut Criterion) {
    c.bench_function("hash_password", |b| b.iter(|| hash_password(black_box("TestPassword123!"))));
}

fn bench_verify_password(c: &mut Criterion) {
    // Pre-hash a password for verification benchmarks
    let hash = hash_password("TestPassword123!").expect("Failed to hash password");

    c.bench_function("verify_password_success", |b| {
        b.iter(|| verify_password(black_box("TestPassword123!"), black_box(&hash)))
    });

    c.bench_function("verify_password_failure", |b| {
        b.iter(|| verify_password(black_box("WrongPassword"), black_box(&hash)))
    });
}

criterion_group!(benches, bench_hash_password, bench_verify_password);
criterion_main!(benches);
