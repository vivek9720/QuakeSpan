use core::fmt;

pub type Result<T> = core::result::Result<T, QSpanError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QSpanError {
    Empty,
    ShortRead { needed: usize, remaining: usize },
    BadMagic,
    BadVersion(u8),
    BadSection(&'static str),
    BadFrame(&'static str),
    BadRules(&'static str),
    LimitExceeded(&'static str),
}

impl fmt::Display for QSpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QSpanError::Empty => write!(f, "empty input"),
            QSpanError::ShortRead { needed, remaining } => {
                write!(f, "short read: needed {needed}, remaining {remaining}")
            }
            QSpanError::BadMagic => write!(f, "bad magic"),
            QSpanError::BadVersion(v) => write!(f, "bad version {v}"),
            QSpanError::BadSection(s) => write!(f, "bad section: {s}"),
            QSpanError::BadFrame(s) => write!(f, "bad frame: {s}"),
            QSpanError::BadRules(s) => write!(f, "bad rules: {s}"),
            QSpanError::LimitExceeded(s) => write!(f, "limit exceeded: {s}"),
        }
    }
}

impl std::error::Error for QSpanError {}
