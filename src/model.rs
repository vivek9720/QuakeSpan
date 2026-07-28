#[derive(Clone, Debug, Default)]
pub struct Archive {
    pub header: Header,
    pub dictionary: Dictionary,
    pub span_map: SpanMap,
    pub sessions: Vec<Session>,
    pub eventlog: Vec<EventEntry>,
    pub programs: Vec<Program>,
    pub blobs: Vec<Vec<u8>>,
    pub diagnostics: Vec<String>,
}

impl Archive {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            ..Self::default()
        }
    }

    pub fn frame_count(&self) -> usize {
        self.sessions
            .iter()
            .map(|session| session.frames.len())
            .sum()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Header {
    pub version: u8,
    pub flags: u8,
    pub captured_at: u64,
    pub bridge_id: u32,
    pub section_count: u16,
}

#[derive(Clone, Debug)]
pub struct Section {
    pub kind: u8,
    pub flags: u8,
    pub offset: usize,
    pub len: usize,
    pub checksum: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Dictionary {
    pub symbols: Vec<Symbol>,
    pub alias_score: u64,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub id: u32,
    pub kind: SymbolKind,
    pub text: String,
    pub score: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Span,
    Joint,
    Sensor,
    Cable,
    Pier,
    Zone,
    Rule,
    Inspection,
    Unknown(u8),
}

impl SymbolKind {
    pub fn code(self) -> u8 {
        match self {
            SymbolKind::Span => 1,
            SymbolKind::Joint => 2,
            SymbolKind::Sensor => 3,
            SymbolKind::Cable => 4,
            SymbolKind::Pier => 5,
            SymbolKind::Zone => 6,
            SymbolKind::Rule => 7,
            SymbolKind::Inspection => 8,
            SymbolKind::Unknown(v) => v,
        }
    }
}

impl Default for SymbolKind {
    fn default() -> Self {
        SymbolKind::Unknown(0)
    }
}

impl From<u8> for SymbolKind {
    fn from(value: u8) -> Self {
        match value {
            1 => SymbolKind::Span,
            2 => SymbolKind::Joint,
            3 => SymbolKind::Sensor,
            4 => SymbolKind::Cable,
            5 => SymbolKind::Pier,
            6 => SymbolKind::Zone,
            7 => SymbolKind::Rule,
            8 => SymbolKind::Inspection,
            other => SymbolKind::Unknown(other),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SpanMap {
    pub joints: Vec<Joint>,
    pub beams: Vec<Beam>,
    pub sensors: Vec<Sensor>,
    pub cables: Vec<Cable>,
    pub zones: Vec<Zone>,
}

#[derive(Clone, Debug, Default)]
pub struct Joint {
    pub id: u32,
    pub kind: u8,
    pub x: i16,
    pub y: i16,
    pub zone: u16,
    pub name_id: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Beam {
    pub id: u32,
    pub from: u32,
    pub to: u32,
    pub length_cm: u16,
    pub camber_mm: i16,
    pub flags: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Sensor {
    pub id: u32,
    pub joint: u32,
    pub channel: u8,
    pub calibration_id: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Cable {
    pub id: u32,
    pub anchor: u32,
    pub beam_a: u32,
    pub beam_b: u32,
    pub tension_kn: u16,
}

#[derive(Clone, Debug, Default)]
pub struct Zone {
    pub id: u16,
    pub name_id: u32,
    pub risk: u8,
    pub max_load_tons: u16,
}

#[derive(Clone, Debug, Default)]
pub struct Session {
    pub id: u32,
    pub started_at: u64,
    pub frames: Vec<Frame>,
    pub checksum: u64,
}

#[derive(Clone, Debug)]
pub enum Frame {
    Strain {
        sensor_id: u32,
        microstrain: i16,
        confidence: u8,
        at: u64,
    },
    Accel {
        sensor_id: u32,
        axis: u8,
        samples: Vec<i16>,
        at: u64,
    },
    JointShift {
        joint_id: u32,
        dx: i16,
        dy: i16,
        severity: u8,
        at: u64,
    },
    CableTension {
        cable_id: u32,
        tension_kn: u16,
        trend: i16,
        at: u64,
    },
    Inspection {
        order_id: u32,
        priority: u8,
        text: String,
        at: u64,
    },
    Heartbeat {
        status: u32,
        at: u64,
    },
    Unknown {
        kind: u8,
        payload: Vec<u8>,
        at: u64,
    },
}

impl Frame {
    pub fn at(&self) -> u64 {
        match self {
            Frame::Strain { at, .. }
            | Frame::Accel { at, .. }
            | Frame::JointShift { at, .. }
            | Frame::CableTension { at, .. }
            | Frame::Inspection { at, .. }
            | Frame::Heartbeat { at, .. }
            | Frame::Unknown { at, .. } => *at,
        }
    }

    pub fn code(&self) -> u8 {
        match self {
            Frame::Strain { .. } => 1,
            Frame::Accel { .. } => 2,
            Frame::JointShift { .. } => 3,
            Frame::CableTension { .. } => 4,
            Frame::Inspection { .. } => 5,
            Frame::Heartbeat { .. } => 6,
            Frame::Unknown { kind, .. } => *kind,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EventEntry {
    pub kind: u8,
    pub key: u32,
    pub clock: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub version: u8,
    pub locals: Vec<i64>,
    pub instructions: Vec<Instruction>,
    pub symbols: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Instruction {
    pub opcode: u8,
    pub operand: i32,
    pub line: u32,
}

#[derive(Clone, Debug, Default)]
pub struct AnalysisReport {
    pub findings: Vec<AnalysisFinding>,
    pub checksum: u64,
    pub frame_count: usize,
    pub span_score: u64,
    pub event_score: u64,
    pub rule_score: u64,
}

#[derive(Clone, Debug)]
pub struct AnalysisFinding {
    pub severity: u8,
    pub code: String,
    pub message: String,
}
