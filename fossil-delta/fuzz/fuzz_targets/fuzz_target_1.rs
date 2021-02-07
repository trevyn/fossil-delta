// rustup default nightly
// cargo install cargo-fuzz
// cargo fuzz run fuzz_target_1 -- -max_len=1000000

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|inp: fossil_delta::DiffInput| {
    let d = fossil_delta::delta(&inp.a, &inp.b);
    let s = fossil_delta::deltainv(&inp.b, &d);
    assert_eq!(&s, &inp.a);
});
