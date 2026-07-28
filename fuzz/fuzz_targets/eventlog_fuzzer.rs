#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = quakespan::replay_eventlog_bytes(data);
});
