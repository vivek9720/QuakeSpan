#[test]
fn archive_seed_decodes_and_analyzes() {
    let data = include_bytes!("../fuzz/corpus/archive_fuzzer/seed_archive.qspn");
    let report = quakespan::decode_and_analyze_archive(data).expect("archive seed should parse");
    assert!(report.frame_count > 0);
}

#[test]
fn entrypoint_seeds_are_wired_to_library_code() {
    let stream = include_bytes!("../fuzz/corpus/stream_fuzzer/seed_stream.qssm");
    let rules = include_bytes!("../fuzz/corpus/rules_fuzzer/seed_rules.qssc");
    let span = include_bytes!("../fuzz/corpus/span_fuzzer/seed_span.qsto");
    let eventlog = include_bytes!("../fuzz/corpus/eventlog_fuzzer/seed_eventlog.qsjr");

    let stream_report = quakespan::decode_stream_and_analyze(stream).expect("stream seed");
    let rule_report = quakespan::run_rule_bytes(rules).expect("rules seed");
    let span_score = quakespan::analyze_span_bytes(span).expect("span seed");
    let event_score = quakespan::replay_eventlog_bytes(eventlog).expect("eventlog seed");

    assert!(stream_report.frame_count > 0);
    assert!(rule_report.steps > 0);
    assert_ne!(span_score, 0);
    assert_ne!(event_score, 0);
}
