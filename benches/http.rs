use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;

async fn webpki() {
    let https = HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let _client: Client<_, hyper::Body> = hyper::Client::builder().build(https);
}

async fn native() {
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let _client: Client<_, hyper::Body> = hyper::Client::builder().build(https);
}

async fn baseline() {
    let _client: Client<_, hyper::Body> = hyper::Client::builder().build_http();
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    c.bench_function("webpki", |b| b.to_async(&rt).iter(|| webpki()));
    c.bench_function("native", |b| b.to_async(&rt).iter(|| native()));
    c.bench_function("baseline", |b| b.to_async(&rt).iter(|| baseline()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
