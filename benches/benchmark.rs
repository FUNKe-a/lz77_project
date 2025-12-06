use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use lz77_project::compress;
use rayon::prelude::*;
use std::hint::black_box;
use std::{env, fs};

const CHUNK_SIZE: usize = 64 * 1024;

fn read_file() -> Vec<u8> {
    let path = env::var("INPUT").expect("missing INPUT env var");
    fs::read(path).expect("read file")
}

fn singlethreaded_lz77(c: &mut Criterion) {
    let whole = read_file();

    c.bench_function("singlethreaded_lz77", |b| {
        b.iter_batched(
            || whole.chunks(CHUNK_SIZE).collect::<Vec<_>>(),
            |chunks| {
                let mut res: Vec<u8> = Vec::new();
                for c in chunks {
                    let mut comp_chunk = compress(&c);
                    res.append(&mut comp_chunk);
                }
                black_box(res)
            },
            BatchSize::SmallInput,
        );
    });
}

fn multithreaded_lz77(c: &mut Criterion) {
    let whole = read_file();

    c.bench_function("multithreaded_lz77", |b| {
        b.iter_batched(
            || whole.chunks(CHUNK_SIZE).collect::<Vec<_>>(),
            |chunks| {
                let mut res: Vec<u8> = Vec::new();
                let comp_chunks = chunks
                    .par_iter()
                    .map(|chunk| compress(chunk))
                    .collect::<Vec<Vec<u8>>>();

                for chunk in comp_chunks {
                    res.extend(chunk);
                }
                black_box(res)
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, singlethreaded_lz77, multithreaded_lz77);
criterion_main!(benches);
