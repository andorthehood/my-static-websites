pub fn flatten_basic_nesting(input: &str) -> String {
    let mut parser = Parser::new(input);
    let mut output = String::with_capacity(input.len());
    while !parser.is_eof() {
        let before = parser.pos;
        if let Some(rule) = parser.parse_rule() {
            emit_rule(&mut output, &rule, "");
        } else {
            // If no rule parsed, consume one char to make progress or break at EOF
            if parser.is_eof() {
                break;
            }
            if parser.pos == before {
                output.push(parser.next_char());
            }
        }
    }
    output
}

#[derive(Debug, Clone)]
struct Rule {
    selector: String,
    content: Vec<Content>,
}

#[derive(Debug, Clone)]
enum Content {
    Decl(String),
    Rule(Rule),
    Raw(char),
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    len: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            pos: 0,
            len: input.len(),
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.len
    }

    fn peek(&self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            Some(self.bytes[self.pos] as char)
        }
    }

    fn next_char(&mut self) -> char {
        let ch = self.bytes[self.pos] as char;
        self.pos += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[start..end]) }
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        self.skip_whitespace();
        // If we are at a block end or delimiter, not a selector start
        match self.peek()? {
            '}' | ';' | '{' => return None,
            _ => {}
        }
        let start = self.pos;
        // A selector must be followed by '{'. Allow newlines and commas inside selector lists.
        while let Some(c) = self.peek() {
            if c == '{' {
                break;
            }
            // Do not early-return on newlines or semicolons here; selectors can span lines
            self.pos += 1;
        }
        if self.peek()? != '{' {
            return None;
        }
        let selector = self.slice(start, self.pos).trim().to_string();
        self.pos += 1; // skip '{'
        let content = self.parse_block_contents();
        Some(Rule { selector, content })
    }

    fn parse_block_contents(&mut self) -> Vec<Content> {
        let mut items = Vec::new();
        loop {
            self.skip_whitespace();
            if self.is_eof() {
                break;
            }
            match self.peek().unwrap() {
                '}' => {
                    self.pos += 1;
                    break;
                }
                '{' => {
                    // orphaned block; copy raw
                    items.push(Content::Raw(self.next_char()));
                }
                '.' | '#' | '[' | '*' | ':' | '@' => {
                    let save = self.pos;
                    if let Some(rule) = self.parse_rule() {
                        items.push(Content::Rule(rule));
                    } else {
                        self.pos = save;
                        items.push(Content::Decl(self.parse_declaration_like()));
                    }
                }
                _ => {
                    items.push(Content::Decl(self.parse_declaration_like()));
                }
            }
        }
        items
    }

    fn parse_declaration_like(&mut self) -> String {
        let start = self.pos;
        let mut in_string: Option<char> = None;
        while let Some(c) = self.peek() {
            if let Some(q) = in_string {
                if c == q {
                    in_string = None;
                }
                self.pos += 1;
                continue;
            }
            // Handle block comments to avoid misinterpreting braces inside comments
            if c == '/' {
                if let Some(next) = self.bytes.get(self.pos + 1).map(|b| *b as char) {
                    if next == '*' {
                        // Skip '/*'
                        self.pos += 2;
                        // Advance until '*/' or EOF
                        while self.pos + 1 < self.len {
                            let a = self.bytes[self.pos] as char;
                            let b = self.bytes[self.pos + 1] as char;
                            self.pos += 1;
                            if a == '*' && b == '/' {
                                self.pos += 1;
                                break;
                            }
                        }
                        continue;
                    }
                }
            }
            match c {
                '\'' | '"' => {
                    in_string = Some(c);
                    self.pos += 1;
                }
                ';' => {
                    self.pos += 1;
                    break;
                }
                '}' => {
                    break;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }
        self.slice(start, self.pos).to_string()
    }
}

fn emit_rule(out: &mut String, rule: &Rule, parent: &str) {
    let selector = rule.selector.trim();
    if selector.is_empty() || selector.contains('&') {
        // unsupported, emit as-is roughly
        out.push_str(selector);
        out.push('{');
        for c in &rule.content {
            emit_content(out, c, parent);
        }
        return;
    }

    // Preserve at-rules (e.g., @media) as blocks and emit children inside without combining selectors
    if selector.starts_with('@') {
        out.push_str(selector);
        out.push('{');
        for c in &rule.content {
            match c {
                Content::Decl(s) => {
                    out.push_str(s);
                }
                Content::Rule(r) => {
                    emit_rule(out, r, "");
                }
                Content::Raw(ch) => {
                    out.push(*ch);
                }
            }
        }
        // Close the at-rule block
        out.push('}');
        return;
    }

    let combined_selector = if parent.is_empty() {
        selector.to_string()
    } else {
        format!("{} {}", parent, selector)
    };
    let mut decls = String::new();
    for c in &rule.content {
        match c {
            Content::Decl(s) => decls.push_str(s),
            _ => {}
        }
    }
    if decls.trim().len() > 0 {
        out.push_str(&combined_selector);
        out.push_str("{");
        out.push_str(decls.trim());
        out.push_str("}");
    }
    for c in &rule.content {
        if let Content::Rule(r) = c {
            emit_rule(out, r, &combined_selector);
        }
    }
}

fn emit_content(out: &mut String, c: &Content, parent: &str) {
    match c {
        Content::Decl(s) => out.push_str(s),
        Content::Rule(r) => emit_rule(out, r, parent),
        Content::Raw(ch) => out.push(*ch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grouped_selectors_preserved() {
        let input = "body,\nhtml,\npre { color: #000000; }";
        let flattened = flatten_basic_nesting(input);
        assert_eq!(flattened, "body,\nhtml,\npre{color: #000000;}");
    }

    #[test]
    fn test_media_query_preserved() {
        let input = "@media(max-width:1500px){\n\t.container{\n\t\twidth:65%;\n\t}\n}";
        let flattened = flatten_basic_nesting(input);
        assert_eq!(
            flattened,
            "@media(max-width:1500px){.container{width:65%;}}"
        );
    }

    #[test]
    fn test_simple_nested_selector_flattened() {
        let input = ".foo { .bar { color: #000000; } }";
        let flattened = flatten_basic_nesting(input);
        assert_eq!(flattened, ".foo .bar{color: #000000;}");
    }

    #[test]
    fn test_element_selector_preserved() {
        let input = "p { margin: 0; padding: 0 0 20px 0; }";
        let flattened = flatten_basic_nesting(input);
        assert_eq!(flattened, "p{margin: 0;padding: 0 0 20px 0;}");
    }

    #[test]
    fn test_descendant_selector_preserved() {
        let input = "article img.loaded { background: initial; aspect-ratio: initial; }";
        let flattened = flatten_basic_nesting(input);
        assert_eq!(
            flattened,
            "article img.loaded{background: initial;aspect-ratio: initial;}"
        );
    }

    #[test]
    fn test_multiple_top_level_rules() {
        let input = ".btn { color: red; }\nh1 { font-weight: 700; }\nbody { margin: 0; }\n";
        let flattened = flatten_basic_nesting(input);
        assert_eq!(
            flattened,
            ".btn{color: red;}h1{font-weight: 700;}body{margin: 0;}"
        );
    }
}
