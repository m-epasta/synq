use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected input: {typ} at line {line}:{col}")]
    Unexpected {
        typ: String,
        line: usize,
        col: usize,
    },
}

impl ParseError {
    pub fn pretty_print(&self, source: &str) -> String {
        match self {
            ParseError::Unexpected { typ, line, col } => {
                let line_idx = *line;
                let col_idx = *col;
                let line_str = source.lines().nth(line_idx).unwrap_or("");
                let padding = " ".repeat(col_idx);
                let pointer = "^".repeat(typ.chars().count().max(1));

                format!(
                    "error: unexpected input '{typ}'\n --> {line}:{col}\n  |\n{line} | {line_str}\n  | {padding}{pointer}"
                )
            }
        }
    }
}
