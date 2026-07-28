#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = quakespan::run_rule_bytes(data);
});
