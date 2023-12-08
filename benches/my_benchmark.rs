use criterion::{criterion_group, criterion_main, Criterion};
use serverless_demo::lambda_functions::process_core_logic;
use serde_json::json;

fn benchmark_core_logic(c: &mut Criterion) {
    let mock_event = json!({
        "key1": "value1",
        "key2": 123,
        "key3": true,
        "key4": {
            "key5": "value5",
            "key6": 456,
            "key7": false
        }
        });

    c.bench_function("process_core_logic", |b| {
        b.iter(|| process_core_logic(mock_event.clone()))
    });
}

criterion_group!(benches, benchmark_core_logic);
criterion_main!(benches);
