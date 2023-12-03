use advents::main_utils::create_day;
// use advents::utils::Task;
use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_advent(c: &mut Criterion) {
    for day in 1..=25 {
        let advent = create_day(day);
        let benchmark_name = format!("day_{day}_");
        let input = format!("./inputs/day{day}.txt");
        c.bench_with_input(
            BenchmarkId::new(benchmark_name.clone() + "_part1", &input),
            &input,
            |b, i| b.iter(|| advent.task_part_one(i)),
        );
        c.bench_with_input(
            BenchmarkId::new(benchmark_name + "_part2", &input),
            &input,
            |b, i| b.iter(|| advent.task_part_two(i)),
        );
    }
}

criterion_group!(benches, bench_advent);
criterion_main!(benches);
