#[macro_use]
extern crate bencher;

use bencher::Bencher;
use uuid::Uuid;

fn as_u128(bench: &mut Bencher) {
    let uuid = Uuid::new_v4();
    let mut v = vec![];
    bench.iter(|| {
        v.push(uuid.as_u128());
    })
}

fn as_fields(bench: &mut Bencher) {
    let uuid = Uuid::new_v4();
    let mut v = vec![];
    bench.iter(|| {
        v.push(uuid.as_fields());
    })
}
fn as_bytes(bench: &mut Bencher) {
    let uuid = Uuid::new_v4();
    let mut v = vec![];
    bench.iter(|| {
        v.push(uuid.as_bytes());
    })
}

benchmark_group!(benches, as_u128, as_fields, as_bytes);
benchmark_main!(benches);
