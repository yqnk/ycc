use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    pub fn to(self, end: Span) -> Span {
        Span {
            start: self.start,
            end: end.end,
        }
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub content: Vec<u8>,

    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn read(path: &str) -> SourceFile {
        let content = fs::read(path).expect("failed to read input file");

        SourceFile::new(path.to_string(), content)
    }

    pub fn new(name: String, content: Vec<u8>) -> SourceFile {
        let mut line_starts = vec![0];
        line_starts.extend(
            content
                .iter()
                .enumerate()
                .filter(|&(_, &b)| b == b'\n')
                .map(|(i, _)| i + 1),
        );

        SourceFile {
            name,
            content,
            line_starts,
        }
    }

    pub fn location(&self, offset: usize) -> (usize, usize) {
        let line = self.line_starts.partition_point(|&start| start <= offset) - 1;

        (line + 1, offset - self.line_starts[line] + 1)
    }

    pub fn fmt_span(&self, span: Span) -> String {
        let (line, col) = self.location(span.start);

        format!("{}:{}:{}", self.name, line, col)
    }

    pub fn snippet(&self, span: Span) -> &str {
        std::str::from_utf8(&self.content[span.start..span.end]).unwrap_or("<invalid utf-8>")
    }
}
