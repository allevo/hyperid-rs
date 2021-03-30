#[macro_use]
extern crate bencher;

use base64::{decode_config, encode_config, URL_SAFE, URL_SAFE_NO_PAD};
use bencher::Bencher;

fn to_url_safe_url_safe_no_pad_be(bench: &mut Bencher) {
    bench.iter(|| {
        let s = encode_config(123456789765432345u128.to_be_bytes(), URL_SAFE_NO_PAD);
        decode_config(s, URL_SAFE_NO_PAD).unwrap();
    })
}
fn to_url_safe_url_safe_no_pad_le(bench: &mut Bencher) {
    bench.iter(|| {
        let s = encode_config(123456789765432345u128.to_le_bytes(), URL_SAFE_NO_PAD);
        decode_config(s, URL_SAFE_NO_PAD).unwrap();
    })
}

fn to_url_safe_url_safe_be(bench: &mut Bencher) {
    bench.iter(|| {
        let s = encode_config(123456789765432345u128.to_be_bytes(), URL_SAFE);
        decode_config(s, URL_SAFE_NO_PAD).unwrap();
    })
}
fn to_url_safe_url_safe_le(bench: &mut Bencher) {
    bench.iter(|| {
        let s = encode_config(123456789765432345u128.to_le_bytes(), URL_SAFE);
        decode_config(s, URL_SAFE_NO_PAD).unwrap();
    })
}

benchmark_group!(
    benches,
    to_url_safe_url_safe_no_pad_be,
    to_url_safe_url_safe_no_pad_le,
    to_url_safe_url_safe_be,
    to_url_safe_url_safe_le
);
benchmark_main!(benches);
