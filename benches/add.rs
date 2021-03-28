#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn test_u8(bench: &mut Bencher) {
    let mut i= 1 as u8;
    bench.iter(|| {
        i = i.checked_add(1)
            .unwrap_or(0)
    })
}

fn test_u16(bench: &mut Bencher) {
    let mut i= 1 as u16;
    bench.iter(|| {
        i = i.checked_add(1)
            .unwrap_or(0)
    })
}

fn test_u32(bench: &mut Bencher) {
    let mut i = 1 as u32;
    bench.iter(|| {
        i = i.checked_add(1)
            .unwrap_or(0)
    })
}

fn test_u64(bench: &mut Bencher) {
    let mut i = 1 as u64;
    bench.iter(|| {
        i = i.checked_add(1)
            .unwrap_or(0)
    })
}

fn test_u128(bench: &mut Bencher) {
    let mut i = 1 as u128;
    bench.iter(|| {
        i = i.checked_add(1)
            .unwrap_or(0)
    })
}

benchmark_group!(benches,test_u8, test_u16, test_u32, test_u64, test_u128);
benchmark_main!(benches);
