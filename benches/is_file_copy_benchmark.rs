use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use is_copy::is_file_copy;

pub fn criterion_benchmark(c: &mut Criterion) {
    let path_dir = Path::new("./data");
    if path_dir.exists() {
        fs::remove_dir_all(path_dir).unwrap();
    }
    fs::create_dir_all(path_dir).unwrap();
    let path_a = path_dir.join("file_a.txt");
    let path_b = path_dir.join("file_b.txt");
    let mut file_a = File::create(&path_a).unwrap();
    let mut file_b = File::create(&path_b).unwrap();
    file_a.write_all(b"this is file a").unwrap();
    file_b.write_all(b"this is file b").unwrap();
    for i in 0..1_000_000 {
        let line = format!("this is line {}\n", i);
        file_a.write_all(line.as_bytes()).unwrap();
        file_b.write_all(line.as_bytes()).unwrap();
    }

    c.bench_function("is_file_copy", |b| {
        b.iter(|| is_file_copy(black_box(&path_a), black_box(&path_b)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
