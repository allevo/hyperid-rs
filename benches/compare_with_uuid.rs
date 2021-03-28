#[macro_use]
extern crate bencher;

use bencher::Bencher;
use hyperid::HyperId;
use uuid::Uuid;

fn uuid(bench: &mut Bencher) {
    bench.iter(|| {
        Uuid::new_v4();
    })
}

fn hyperid(bench: &mut Bencher) {
    let mut hyperid = HyperId::default();
    bench.iter(|| {
        hyperid.generate();
    })
}

benchmark_group!(benches, uuid, hyperid);
benchmark_main!(benches);
