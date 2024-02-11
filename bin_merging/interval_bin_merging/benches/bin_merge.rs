use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_library::CountAggregateData;
use project_library::bin::Bin;
use project_library::aggregation_strategy::AggregationStrategy;
use rand::Rng;

fn benchmark_categorise_recent_bins(c: &mut Criterion) {
    c.bench_function("categorise_recent_bins", |b| {
        let mut data = CountAggregateData::new();
        let mut rng = rand::thread_rng();

        for i in 0..10000 {
            let random_mean: f64 = rng.gen();  
            data.x_stats.push( Bin {mean: random_mean , sum: 0.0 , min: 0.0, max: 0.0, count: 0, timestamp: 0 } );
            data.y_stats.push( Bin {mean: (random_mean*2.0) , sum: 0.0 , min: 0.0, max: 0.0, count: 0, timestamp: 0 } );
        }

        b.iter(|| data.categorise_recent_bins(black_box(5000))); // Example usage
    });
}

criterion_group!(benches, benchmark_categorise_recent_bins);
criterion_main!(benches);