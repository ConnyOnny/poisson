#![feature(test)]

extern crate test;
use test::{Bencher, black_box};

extern crate poisson;
use poisson::{PoissonDisk, Ebeida, Bridson};

extern crate rand;
use rand::{SeedableRng, XorShiftRng};

extern crate nalgebra as na;
pub type Vect = na::Vec4<f64>;

#[bench]
fn bench_ebeida_4d_1_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let poisson =
        PoissonDisk::<_, Vect>::with_samples(1, 0.8, PoissonType::Normal)
            .build(rand::weak_rng(), Ebeida);
    b.iter(|| black_box(poisson.generate()));
}

#[bench]
fn bench_ebeida_4d_10_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let poisson =
        PoissonDisk::<_, Vect>::with_samples(10, 0.8, PoissonType::Normal)
            .build(rand::weak_rng(), Ebeida);
    b.iter(|| black_box(poisson.generate()));
}

#[bench]
fn bench_ebeida_4d_100_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let poisson =
        PoissonDisk::<_, Vect>::with_samples(100, 0.8, PoissonType::Normal)
            .build(rand::weak_rng(), Ebeida);
    b.iter(|| black_box(poisson.generate()));
}

#[bench]
fn bench_bridson_4d_1_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let poisson =
        PoissonDisk::<_, Vect>::with_samples(1, 0.8, PoissonType::Normal)
            .build(rand::weak_rng(), Bridson);
    b.iter(|| black_box(poisson.generate()));
}

#[bench]
fn bench_bridson_4d_10_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let poisson =
        PoissonDisk::<_, Vect>::with_samples(10, 0.8, PoissonType::Normal)
            .build(rand::weak_rng(), Bridson);
    b.iter(|| black_box(poisson.generate()));
}

#[bench]
fn bench_bridson_4d_100_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let poisson =
        PoissonDisk::<_, Vect>::with_samples(100, 0.8, PoissonType::Normal)
            .build(rand::weak_rng(), Bridson);
    b.iter(|| black_box(poisson.generate()));
}
