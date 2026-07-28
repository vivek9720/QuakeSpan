use crate::catalog;
use crate::checksum;
use crate::fastpath;
use crate::model::{AnalysisFinding, AnalysisReport, Archive, Frame};
use crate::parser;
use crate::rulebook::{self, RuleContext};

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, archive: &Archive) -> AnalysisReport {
        let mut report = AnalysisReport::default();
        report.frame_count = archive.frame_count();
        let mut span_map = archive.span_map.clone();
        report.span_score = parser::score_span_map(&mut span_map, archive.dictionary.alias_score);
        let mut eventlog = archive.eventlog.clone();
        report.event_score = parser::replay_eventlog(&mut eventlog);
        for program in &archive.programs {
            if let Ok(exec) = parser::run_rules(program) {
                report.rule_score ^= exec.output ^ exec.steps;
            }
        }
        for session in &archive.sessions {
            report.checksum ^= session.checksum;
            for frame in &session.frames {
                report.checksum ^= frame_score(frame);
            }
        }
        let mut cache = vec![
            archive.header.captured_at,
            archive.header.bridge_id as u64,
            report.span_score,
            report.event_score,
            report.rule_score,
            report.checksum,
            archive.dictionary.alias_score,
            report.frame_count as u64,
            archive.blobs.len() as u64,
            archive.sessions.len() as u64,
            archive.programs.len() as u64,
            archive.eventlog.len() as u64,
        ];
        report.checksum ^= fastpath::fast_replay_cache(&mut cache, report.checksum);
        report.checksum ^= catalog::score_catalog(
            report.span_score ^ archive.dictionary.alias_score,
            archive.header.bridge_id,
        );
        let ctx = RuleContext {
            bridge_id: archive.header.bridge_id as u64,
            symbol_score: archive.dictionary.alias_score,
            span_score: report.span_score,
            event_score: report.event_score,
            session_score: report.checksum,
            rule_score: report.rule_score,
            frame_count: report.frame_count as u64,
            section_count: archive.header.section_count as u64,
        };
        for finding in rulebook::evaluate_all(&ctx) {
            report.findings.push(AnalysisFinding {
                severity: finding.severity,
                code: finding.code.to_string(),
                message: format!("rule detail {:#x}", finding.detail),
            });
        }
        if archive.dictionary.symbols.is_empty() {
            report.findings.push(AnalysisFinding {
                severity: 1,
                code: "EMPTY_DICTIONARY".to_string(),
                message: "archive has no symbol dictionary".to_string(),
            });
        }
        report
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn frame_score(frame: &Frame) -> u64 {
    match frame {
        Frame::Strain {
            sensor_id,
            microstrain,
            confidence,
            at,
        } => checksum::mix_u64(
            *sensor_id as u64 ^ *at ^ ((*confidence as u64) << 16) ^ (*microstrain as i64 as u64),
        ),
        Frame::Accel {
            sensor_id,
            axis,
            samples,
            at,
        } => samples.iter().fold(
            *sensor_id as u64 ^ ((*axis as u64) << 40) ^ *at,
            |acc, s| acc.wrapping_add(*s as i64 as u64).rotate_left(3),
        ),
        Frame::JointShift {
            joint_id,
            dx,
            dy,
            severity,
            at,
        } => checksum::mix_u64(
            *joint_id as u64
                ^ ((*dx as i64 as u64) << 5)
                ^ ((*dy as i64 as u64) << 19)
                ^ ((*severity as u64) << 41)
                ^ *at,
        ),
        Frame::CableTension {
            cable_id,
            tension_kn,
            trend,
            at,
        } => checksum::mix_u64(
            *cable_id as u64 ^ ((*tension_kn as u64) << 13) ^ ((*trend as i64 as u64) << 29) ^ *at,
        ),
        Frame::Inspection {
            order_id,
            priority,
            text,
            at,
        } => {
            checksum::rolling64(text.as_bytes())
                ^ *order_id as u64
                ^ ((*priority as u64) << 33)
                ^ *at
        }
        Frame::Heartbeat { status, at } => *status as u64 ^ *at,
        Frame::Unknown { kind, payload, at } => {
            checksum::rolling64(payload) ^ ((*kind as u64) << 56) ^ *at
        }
    }
}
