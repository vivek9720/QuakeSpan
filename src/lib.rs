//! QuakeSpan decodes disconnected bridge structural-health monitoring bundles.

pub mod analysis;
pub mod catalog;
pub mod checksum;
pub mod cursor;
pub mod error;
pub mod fastpath;
pub mod model;
pub mod parser;
pub mod rulebook;

pub use analysis::Analyzer;
pub use error::{QSpanError, Result};
pub use model::{AnalysisFinding, AnalysisReport, Archive, Header, Session, SpanMap};

pub fn parse_archive(data: &[u8]) -> Result<Archive> {
    parser::parse_archive(data)
}

pub fn decode_and_analyze_archive(data: &[u8]) -> Result<AnalysisReport> {
    let archive = parse_archive(data)?;
    Ok(Analyzer::new().analyze(&archive))
}

pub fn decode_stream(data: &[u8]) -> Result<Archive> {
    parser::decode_stream(data)
}

pub fn decode_stream_and_analyze(data: &[u8]) -> Result<AnalysisReport> {
    let archive = decode_stream(data)?;
    Ok(Analyzer::new().analyze(&archive))
}

pub fn run_rule_bytes(data: &[u8]) -> Result<parser::ExecutionReport> {
    let program = parser::compile_rules(data)?;
    parser::run_rules(&program)
}

pub fn replay_eventlog_bytes(data: &[u8]) -> Result<u64> {
    let mut events = parser::parse_eventlog(data)?;
    Ok(parser::replay_eventlog(&mut events))
}

pub fn analyze_span_bytes(data: &[u8]) -> Result<u64> {
    let dict = model::Dictionary::default();
    let mut map = parser::parse_span_map(data, &dict)?;
    Ok(parser::score_span_map(&mut map, dict.alias_score))
}
