use std::any::type_name;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use p3_baby_bear::BabyBear;
use p3_field::extension::Complex;
use p3_field::TwoAdicField;
use p3_fri::fold_even_odd;
use p3_fri::yu_fold_poly;
use p3_fri::fold_poly_with_dft;
use p3_goldilocks::Goldilocks;
use p3_mersenne_31::Mersenne31;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

fn bench<F: TwoAdicField>(c: &mut Criterion, log_sizes: &[usize])
where
    Standard: Distribution<F>,
{
    let name = format!("fold_even_odd::<{}>", type_name::<F>(),);
    let mut group = c.benchmark_group(&name);
    group.sample_size(10);

    for log_size in log_sizes {
        let n = 1 << log_size;

        let mut rng = thread_rng();
        let beta = rng.sample(Standard);
        let poly = rng.sample_iter(Standard).take(n).collect_vec();

        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter(|| {
                fold_even_odd(poly.clone(), beta);
            })
        });
    }
}

fn bench_yu<F: TwoAdicField>(c: &mut Criterion, log_sizes: &[usize], folding_factors: &[usize])
where
    Standard: Distribution<F>,
{
    let name = format!("fold_even_odd::<{}>", type_name::<F>(),);
    let mut group = c.benchmark_group(&name);
    group.sample_size(10);

    for log_size in log_sizes {
        for folding_factor in folding_factors{
            let n = 1 << log_size;

            let mut rng = thread_rng();
            let beta = rng.sample(Standard);
            let poly = rng.sample_iter(Standard).take(n).collect_vec();
    
            group.bench_function(BenchmarkId::from_parameter(n), |b| {
                b.iter(|| {
                    yu_fold_poly(poly.clone(), beta, *folding_factor);
                })
            });
        }
    }
}


fn bench_dft<F: TwoAdicField>(c: &mut Criterion, log_sizes: &[usize], folding_factors: &[usize])
where
    Standard: Distribution<F>,
{
    let name = format!("fold_even_odd::<{}>", type_name::<F>(),);
    let mut group = c.benchmark_group(&name);
    group.sample_size(10);

    for log_size in log_sizes {
        for folding_factor in folding_factors{
            let n = 1 << log_size;

            let mut rng = thread_rng();
            let beta = rng.sample(Standard);
            let poly = rng.sample_iter(Standard).take(n).collect_vec();
    
            group.bench_function(BenchmarkId::from_parameter(n), |b| {
                b.iter(|| {
                    fold_poly_with_dft(poly.clone(), beta, *folding_factor);
                })
            });
        }
    }
}

fn bench_fold_even_odd(c: &mut Criterion) {
    let log_sizes = [12, 14, 16, 18, 20, 22];

    bench::<BabyBear>(c, &log_sizes);
    bench::<Goldilocks>(c, &log_sizes);
    bench::<Complex<Mersenne31>>(c, &log_sizes);
}

fn bench_yu_fold_poly(c: &mut Criterion) {
    let log_sizes = [12, 14, 16, 18, 20, 22];
    let folding_factors = [2, 4, 8, 16];

    bench_yu::<BabyBear>(c, &log_sizes, &folding_factors);
    bench_yu::<Goldilocks>(c, &log_sizes, &folding_factors);
    bench_yu::<Complex<Mersenne31>>(c, &log_sizes, &folding_factors);
}

fn bench_fold_poly_with_dft(c: &mut Criterion) {
    let log_sizes = [12, 14, 16, 18, 20, 22];
    let folding_factors = [2, 4, 8, 16];

    bench_dft::<BabyBear>(c, &log_sizes, &folding_factors);
    bench_dft::<Goldilocks>(c, &log_sizes, &folding_factors);
    bench_dft::<Complex<Mersenne31>>(c, &log_sizes, &folding_factors);
}

criterion_group!(benches, bench_fold_even_odd, bench_yu_fold_poly, bench_fold_poly_with_dft);
criterion_main!(benches);
