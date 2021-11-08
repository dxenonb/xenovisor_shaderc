#[derive(Debug, Clone)]
pub struct Annotations {
    pub sources: Vec<String>,
    pub primary: Span,
    pub annotations: Vec<SourceAnnotation>,
}

impl Annotations {
    pub fn from_error_message(s: String) -> Annotations {
        Annotations {
            sources: vec![],
            primary: Span { start: SourcePos::new(0, 0), end: SourcePos::new(0, 0) },
            annotations: vec![
                SourceAnnotation {
                    source: None,
                    message: SpanMessage::Error(s),
                    annotations: vec![],
                }
            ]
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceAnnotation {
    pub source: Option<usize>,
    pub message: SpanMessage,
    pub annotations: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: SourcePos,
    pub end: SourcePos,
}

#[derive(Debug, Clone)]
pub struct SourcePos {
    pub line: u32,
    pub col: u32,
}

impl SourcePos {
    fn new(line: u32, col: u32) -> SourcePos {
        SourcePos {
            line,
            col,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpanMessage {
    Error(String),
    Help(String),
    Note(String),
    Warning(String),
}
