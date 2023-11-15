#[derive(Clone)]
pub struct SyntaxHighlight {
    pub language: String,
    pub file_name_patterns: Vec<String>,
    pub keywords: Vec<String>,
    pub types: Vec<String>,
    pub single_line_comment: String,
}

#[derive(Clone, Debug)]
pub enum HLGroup {
    NORMAL,
    NUMBER,
    STRING,
    KEYWORD,
    TYPE,
    COMMENT,
}

pub fn hl_group_to_term_color(hl_group: &HLGroup) -> u8 {
    match hl_group {
        HLGroup::NORMAL => 0,
        HLGroup::NUMBER => 31,
        HLGroup::STRING => 32,
        HLGroup::KEYWORD => 33,
        HLGroup::TYPE => 35,
        HLGroup::COMMENT => 36,
    }
}

pub fn get_syntax_highlighting(file_path: &str) -> Option<SyntaxHighlight> {
    let database = vec![SyntaxHighlight {
        language: "rust".to_string(),
        file_name_patterns: vec![".rs".to_string()],
        keywords: vec![
            "SelfTy", "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
            "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
            "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super",
            "trait", "true", "type", "union", "unsafe", "use", "where", "while",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        types: vec![
            "array",
            "bool",
            "char",
            "f32",
            "f64",
            "fn",
            "u8",
            "u16",
            "u32",
            "u64",
            "u128",
            "i8",
            "i16",
            "i32",
            "i64",
            "i128",
            "isize",
            "pointer",
            "reference",
            "slice",
            "str",
            "tuple",
            "u8",
            "u16",
            "u32",
            "u64",
            "u128",
            "unit",
            "usize",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        single_line_comment: "//".to_string(),
    }];

    for highlight in &database {
        for pattern in &highlight.file_name_patterns {
            if file_path.ends_with(pattern) {
                return Some(highlight.clone());
            }
        }
    }

    None
}

fn is_number(c: u8) -> bool {
    let c = c as char;
    '0' <= c && c <= '9'
}

fn is_alpha(c: u8) -> bool {
    let c = c as char;
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
}

fn is_alphanumeric(c: u8) -> bool {
    is_number(c) || is_alpha(c)
}

struct Lexer<'a> {
    start: usize,
    current: usize,
    data: Vec<u8>,
    syntax_hl: &'a SyntaxHighlight,
    result: Vec<HLGroup>,
}

impl<'a> Lexer<'a> {
    fn new(data: Vec<u8>, highlight: &'a SyntaxHighlight) -> Self {
        let len = data.len();

        Self {
            start: 0,
            current: 0,
            data,
            syntax_hl: highlight,
            result: vec![HLGroup::NORMAL; len],
        }
    }

    fn get_substring(&self, a: usize, b: usize) -> String {
        String::from_utf8(self.data[a..b].iter().cloned().collect::<Vec<_>>()).unwrap()
    }

    fn get_current_token(&self) -> String {
        self.get_substring(self.start, self.current)
    }

    fn is_start_of_comment(&self) -> bool {
        self.get_substring(self.start, self.data.len())
            .starts_with(&self.syntax_hl.single_line_comment)
    }

    fn set_hl_group(&mut self, hl_group: HLGroup) {
        for i in self.start..self.current {
            self.result[i] = hl_group.clone();
        }
    }

    fn advance(&mut self) -> u8 {
        let c = self.data[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> u8 {
        if self.at_the_end() {
            return b'\0';
        }
        self.data[self.current]
    }

    fn at_the_end(&self) -> bool {
        self.current >= self.data.len()
    }

    fn scan_number(&mut self) {
        while !self.at_the_end() && is_number(self.peek()) {
            self.advance();
        }
        self.set_hl_group(HLGroup::NUMBER);
    }

    fn scan_identifier(&mut self) {
        while !self.at_the_end() && is_alphanumeric(self.peek()) {
            self.advance();
        }

        let current_token = self.get_current_token();
        if self.syntax_hl.keywords.contains(&current_token) {
            self.set_hl_group(HLGroup::KEYWORD);
        }
        if self.syntax_hl.types.contains(&current_token) {
            self.set_hl_group(HLGroup::TYPE);
        }
    }

    fn scan_comment(&mut self) {
        self.current = self.data.len();
        self.set_hl_group(HLGroup::COMMENT);
    }

    fn scan_string(&mut self, quote: u8) {
        while !self.at_the_end() {
            let c = self.advance();

            if c == b'\\' && !self.at_the_end() {
                self.advance();
            }
            if c == quote {
                break;
            }
        }
        self.set_hl_group(HLGroup::STRING);
    }

    fn scan(&mut self) {
        let c = self.advance();

        if is_number(c) {
            self.scan_number();
        } else if is_alpha(c) {
            self.scan_identifier();
        } else if self.is_start_of_comment() {
            self.scan_comment();
        } else if [b'"', b'\''].contains(&c) {
            self.scan_string(c)
        } else {
            ()
        }
    }

    fn tokenize(&mut self) -> Vec<HLGroup> {
        while self.current < self.data.len() {
            self.start = self.current;
            self.scan();
        }

        self.result.clone()
    }
}

pub fn get_line_highlighting(
    data: &Vec<u8>,
    maybe_syntax_hl: &Option<SyntaxHighlight>,
) -> Vec<HLGroup> {
    if let Some(syntax_hl) = maybe_syntax_hl {
        Lexer::new(data.clone(), syntax_hl).tokenize()
    } else {
        vec![HLGroup::NORMAL; data.len()]
    }
}
