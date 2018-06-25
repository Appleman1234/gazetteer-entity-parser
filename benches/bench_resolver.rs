#[macro_use]
extern crate criterion;
extern crate nr_builtin_resolver;
extern crate rand;

use rand::Rng;
use rand::{thread_rng};
use rand::seq::sample_iter;
use rand::distributions::Alphanumeric;
use nr_builtin_resolver::{Resolver, Gazetteer, EntityValue};

use criterion::Criterion;

/// Function generating a random string representing a single word of various length
fn generate_random_string(rng: &mut rand::ThreadRng) -> String {
    let n_char = rng.gen_range(3, 8);
    rng.sample_iter(&Alphanumeric).take(n_char).collect()
}


/// Random string generator with a bit of redundancy to make it harder for the resolver
struct RandomStringGenerator {
    unique_strings: Vec<String>,
    rng: rand::ThreadRng
}

impl RandomStringGenerator {
    fn new(n_unique_strings: usize) -> RandomStringGenerator {
        let mut rng = thread_rng();
        let unique_strings = (0..n_unique_strings).map(|_| generate_random_string(&mut rng)).collect();
        RandomStringGenerator { unique_strings, rng: rng }
    }

    fn generate(&mut self) -> String {
        let n_words = self.rng.gen_range(1, 4);
        let mut s: Vec<String> = vec![];
        for sample_idx in sample_iter(&mut self.rng, 0..self.unique_strings.len(), n_words).unwrap() {
            s.push(self.unique_strings.get(sample_idx).unwrap().to_string())
        }
        s.join(" ")
    }

}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rsg = RandomStringGenerator::new(100);
    let mut gazetteer = Gazetteer { data: Vec::new() };
    for _ in 1..100000 {
        let val = rsg.generate();
        gazetteer.add(EntityValue {
            raw_value: val.clone(),
            resolved_value: val.to_lowercase(),
        });
    }
    let resolver = Resolver::from_gazetteer(&gazetteer, 0.5).unwrap();

    c.bench_function("Resolve random value", move |b| b.iter(|| resolver.run(&rsg.generate())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
