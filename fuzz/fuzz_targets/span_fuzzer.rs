#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = quakespan::analyze_span_bytes(data);
});
