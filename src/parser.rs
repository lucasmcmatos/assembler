use std::fs;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    Label,
}

pub struct Parser {
    lines: Vec<String>,
    current: usize,
}

impl Parser {
    pub fn new(filename: &str) -> io::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let lines = contents
            .lines()
            .map(Self::clean_line)
            .filter(|line| !line.is_empty())
            .collect();
        Ok(Parser { lines, current: 0 })
    }

    pub fn has_more_instructions(&self) -> bool {
        self.current < self.lines.len()
    }

    pub fn advance(&mut self) -> &str {
        let line = &self.lines[self.current];
        self.current += 1;
        line
    }

    pub fn instruction_type(line: &str) -> InstructionType {
        if line.starts_with('@') {
            InstructionType::AInstruction
        } else if line.starts_with('(') {
            InstructionType::Label
        } else {
            InstructionType::CInstruction
        }
    }

    fn clean_line(line: &str) -> String {
        let without_comment = match line.find("//") {
            Some(idx) => &line[..idx],
            None => line,
        };
        without_comment.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn write_temp_asm(contents: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "assembler_parser_test_{}_{id}.asm",
            std::process::id()
        ));
        fs::write(&path, contents).expect("failed to write temp .asm file");
        path
    }

    #[test]
    fn strips_comments_and_blank_lines() {
        let path = write_temp_asm(
            "// comentário inicial\n\
             @2\n\
             D=A\n\
             \n\
             // outro comentário\n\
             @3\n\
             D=D+A\n",
        );
        let mut parser = Parser::new(path.to_str().unwrap()).unwrap();

        let mut collected = Vec::new();
        while parser.has_more_instructions() {
            collected.push(parser.advance().to_string());
        }

        assert_eq!(collected, vec!["@2", "D=A", "@3", "D=D+A"]);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn strips_inline_comment_without_leading_space() {
        let path = write_temp_asm("D=A//comentário colado\n");
        let mut parser = Parser::new(path.to_str().unwrap()).unwrap();

        assert!(parser.has_more_instructions());
        assert_eq!(parser.advance(), "D=A");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn ignores_whitespace_only_lines() {
        let path = write_temp_asm("   \n\t\n@1\n   \t  \n");
        let mut parser = Parser::new(path.to_str().unwrap()).unwrap();

        assert!(parser.has_more_instructions());
        assert_eq!(parser.advance(), "@1");
        assert!(!parser.has_more_instructions());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn has_more_instructions_is_false_once_exhausted() {
        let path = write_temp_asm("@1\n");
        let mut parser = Parser::new(path.to_str().unwrap()).unwrap();

        assert!(parser.has_more_instructions());
        parser.advance();
        assert!(!parser.has_more_instructions());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn new_returns_err_for_missing_file() {
        let result = Parser::new("/path/that/does/not/exist.asm");
        assert!(result.is_err());
    }

    #[test]
    fn recognizes_a_instruction() {
        assert_eq!(
            Parser::instruction_type("@2"),
            InstructionType::AInstruction
        );
    }

    #[test]
    fn recognizes_label() {
        assert_eq!(Parser::instruction_type("(LOOP)"), InstructionType::Label);
    }

    #[test]
    fn recognizes_c_instruction() {
        assert_eq!(
            Parser::instruction_type("D=A"),
            InstructionType::CInstruction
        );
    }
}
