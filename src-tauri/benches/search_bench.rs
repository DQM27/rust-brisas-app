//! Benchmarks for search/indexing functions (Tantivy)
//!
//! Run with: cargo bench --bench search_bench

use brisas_app_lib::search::{schema, searcher};
use brisas_app_lib::services::search_service::SearchService;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;

fn bench_search_service_init(c: &mut Criterion) {
    c.bench_function("search_service_init", |b| b.iter(|| SearchService::test_instance()));
}

fn bench_search_query(c: &mut Criterion) {
    // Initialize search service once
    let service = Arc::new(SearchService::test_instance());

    c.bench_function("search_empty_index", |b| {
        b.iter(|| service.search(black_box("test"), black_box(10)))
    });
}

fn bench_schema_creation(c: &mut Criterion) {
    c.bench_function("create_search_schema", |b| b.iter(|| schema::create_search_schema()));
}

criterion_group!(benches, bench_search_service_init, bench_search_query, bench_schema_creation);
criterion_main!(benches);
