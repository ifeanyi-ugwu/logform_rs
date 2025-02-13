use criterion::{black_box, criterion_group, criterion_main, Criterion};
use logform::{json, new::Format as _, timestamp, LogInfo};
use serde_json::Value;

fn old_json_format_benchmark(c: &mut Criterion) {
    let formatter = json(); // Old version of JSON formatting
    let info = LogInfo::new("info", "Benchmarking old JSON")
        .with_meta("user_id", 12345)
        .with_meta("session_id", "abcde12345");

    c.bench_function("Old JSON Format", |b| {
        b.iter(|| {
            let _ = formatter.transform(black_box(info.clone()), None);
        })
    });
}

fn new_json_format_benchmark(c: &mut Criterion) {
    let formatter = logform::new::json::JsonFormat; // New version of JSON formatting
    let info = LogInfo::new("info", "Benchmarking new JSON")
        .with_meta("user_id", Value::Number(12345.into()))
        .with_meta("session_id", Value::String("abcde12345".to_string()));

    c.bench_function("New JSON Format", |b| {
        b.iter(|| {
            let _ = formatter.transform(black_box(info.clone()));
        })
    });
}

fn old_timestamp_format_benchmark(c: &mut Criterion) {
    let formatter = timestamp(); // Old timestamp formatter
    let info = LogInfo::new("info", "Benchmarking old timestamp");

    c.bench_function("Old Timestamp Format", |b| {
        b.iter(|| {
            let _ = formatter.transform(black_box(info.clone()), None);
        })
    });
}

fn new_timestamp_format_benchmark(c: &mut Criterion) {
    let formatter = logform::new::timestamp::Timestamp::new(); // New timestamp formatter
    let info = LogInfo::new("info", "Benchmarking new timestamp");

    c.bench_function("New Timestamp Format", |b| {
        b.iter(|| {
            let _ = formatter.transform(black_box(info.clone()));
        })
    });
}

criterion_group!(
    benches,
    old_json_format_benchmark,
    new_json_format_benchmark,
    old_timestamp_format_benchmark,
    new_timestamp_format_benchmark
);
criterion_main!(benches);
