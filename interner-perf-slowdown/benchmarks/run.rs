use std::ffi::OsString;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::hash::{BuildHasher, BuildHasherDefault};
use walkdir::WalkDir;

fn do_stuff<RS: Default + BuildHasher>() {
    let arena = typed_arena::Arena::new();
    let mut basic_interner = basic_interner::Interner::<RS>::new(&arena);
    // FIXME: Use local path instead of using hard-coded path.
    find_rs_files(
        "./benchmarks/data/rust-analyzer",
        &mut basic_interner,
    );
}

fn warm_cache() {
    for _ in 0..10 {
        do_stuff::<ahash::RandomState>();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    criterion_benchmark_generic::<ahash::RandomState>(c, "ahash");
    criterion_benchmark_generic::<BuildHasherDefault<rustc_hash::FxHasher>>(c, "fxhash");
}

fn criterion_benchmark_generic<RS: Send + Sync + Clone + Default + BuildHasher>(
    c: &mut Criterion,
    hash_name: &'static str,
) {
    let benchmark_name = |n: u8, cache_state: &'static str| {
        format!(
            "intern {}: ({}, n = {}, {})",
            if n == 1 { "serial" } else { "parallel" },
            cache_state,
            n,
            hash_name
        )
    };

    warm_cache();

    c.bench_function(&benchmark_name(1, "warm"), |b| {
        b.iter_batched(|| {}, |_| do_stuff::<RS>(), BatchSize::NumIterations(1));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn find_rs_files<'a, RS: Default + BuildHasher>(
    path: &str,
    interner: &mut basic_interner::Interner<'a, RS>,
) {
    let mut rs = OsString::new();
    rs.push("rs");
    for pb in WalkDir::new(path).into_iter().filter_map(|e| {
        let pb = e.ok()?.into_path();
        if pb.extension() == Some(&rs) {
            Some(pb)
        } else {
            None
        }
    }) {
        let buf = std::fs::read_to_string(&pb).unwrap();
        for str in buf.split_whitespace() {
            interner.intern(str);
        }
    }
}

mod basic_interner {
    use std::collections::HashMap;
    use std::hash::BuildHasher;
    use typed_arena::Arena;

    pub struct Interner<'a, RS> {
        map: HashMap<&'a str, u32, RS>,
        vec: Vec<&'a str>,
        arena: &'a Arena<u8>,
    }

    impl<RS: Default + BuildHasher> Interner<'_, RS> {
        pub fn new(arena: &Arena<u8>) -> Interner<RS> {
            Interner {
                map: HashMap::with_capacity_and_hasher(1024, Default::default()),
                vec: Vec::new(),
                arena,
            }
        }

        pub fn intern(&mut self, name: &str) -> u32 {
            if let Some(&idx) = self.map.get(name) {
                return idx;
            }
            let idx = self.vec.len() as u32;
            let name = self.arena.alloc_str(name);
            self.map.insert(name, idx);
            self.vec.push(name);
            debug_assert!(self.lookup(idx) == name);
            debug_assert!(self.intern(name) == idx);
            idx
        }

        pub fn lookup(&self, idx: u32) -> &str {
            self.vec[idx as usize]
        }
    }
}
