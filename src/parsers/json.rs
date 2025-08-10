use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    String(String),
    Integer(i64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[allow(dead_code)]
pub struct JsonParser {
    chars: Vec<char>,
    pos: usize,
}

#[allow(dead_code)]
impl JsonParser {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();

        if self.pos >= self.chars.len() {
            return Err("Unexpected end of input".to_string());
        }

        match self.current_char() {
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            c if c.is_ascii_digit() || c == '-' => self.parse_number(),
            _ => Err(format!("Unexpected character: {}", self.current_char())),
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, String> {
        if self.current_char() != '"' {
            return Err("Expected opening quote".to_string());
        }

        self.advance(); // Skip opening quote
        let mut value = String::new();

        while self.pos < self.chars.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                value.push(self.current_char()); // Keep the backslash
                self.advance();
                if self.pos < self.chars.len() {
                    value.push(self.current_char()); // Keep the escaped character
                    self.advance();
                }
            } else {
                value.push(self.current_char());
                self.advance();
            }
        }

        if self.pos >= self.chars.len() {
            return Err("Unterminated string".to_string());
        }

        self.advance(); // Skip closing quote

        Ok(JsonValue::String(value))
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let mut number_str = String::new();

        if self.current_char() == '-' {
            number_str.push(self.current_char());
            self.advance();
        }

        if !self.current_char().is_ascii_digit() {
            return Err("Invalid number format".to_string());
        }

        while self.pos < self.chars.len() && self.current_char().is_ascii_digit() {
            number_str.push(self.current_char());
            self.advance();
        }

        number_str
            .parse::<i64>()
            .map(JsonValue::Integer)
            .map_err(|_| "Invalid integer".to_string())
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        if self.current_char() != '[' {
            return Err("Expected opening bracket".to_string());
        }

        self.advance(); // Skip '['
        self.skip_whitespace();

        let mut elements = Vec::new();

        if self.current_char() == ']' {
            self.advance();
            return Ok(JsonValue::Array(elements));
        }

        loop {
            elements.push(self.parse_value()?);
            self.skip_whitespace();

            match self.current_char() {
                ',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                ']' => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or ']' in array".to_string()),
            }
        }

        Ok(JsonValue::Array(elements))
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        if self.current_char() != '{' {
            return Err("Expected opening brace".to_string());
        }

        self.advance(); // Skip '{'
        self.skip_whitespace();

        let mut object = HashMap::new();

        if self.current_char() == '}' {
            self.advance();
            return Ok(JsonValue::Object(object));
        }

        loop {
            // Parse key
            let key = match self.parse_value()? {
                JsonValue::String(s) => s,
                _ => return Err("Object key must be a string".to_string()),
            };

            self.skip_whitespace();

            if self.current_char() != ':' {
                return Err("Expected ':' after object key".to_string());
            }

            self.advance(); // Skip ':'
            self.skip_whitespace();

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();

            match self.current_char() {
                ',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                '}' => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or '}' in object".to_string()),
            }
        }

        Ok(JsonValue::Object(object))
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn current_char(&self) -> char {
        self.chars.get(self.pos).copied().unwrap_or('\0')
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}

#[allow(dead_code)]
pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let mut parser = JsonParser::new(input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let result = parse_json("\"hello world\"").unwrap();
        assert_eq!(result, JsonValue::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_string_with_whitespace() {
        let result = parse_json("\"  hello   world  \"").unwrap();
        assert_eq!(result, JsonValue::String("  hello   world  ".to_string()));

        let result = parse_json("\"\\t\\n\\r\"").unwrap();
        assert_eq!(result, JsonValue::String("\\t\\n\\r".to_string()));
    }

    #[test]
    fn test_parse_json_with_string_containing_whitespace() {
        let result = parse_json(
            "{\"message\": \"  hello   world  \", \"data\": [\"  item1  \", \"  item2  \"]}",
        )
        .unwrap();
        let mut expected = HashMap::new();
        expected.insert(
            "message".to_string(),
            JsonValue::String("  hello   world  ".to_string()),
        );
        expected.insert(
            "data".to_string(),
            JsonValue::Array(vec![
                JsonValue::String("  item1  ".to_string()),
                JsonValue::String("  item2  ".to_string()),
            ]),
        );
        assert_eq!(result, JsonValue::Object(expected));
    }

    #[test]
    fn test_parse_integer() {
        let result = parse_json("42").unwrap();
        assert_eq!(result, JsonValue::Integer(42));

        let result = parse_json("-123").unwrap();
        assert_eq!(result, JsonValue::Integer(-123));
    }

    #[test]
    fn test_parse_empty_array() {
        let result = parse_json("[]").unwrap();
        assert_eq!(result, JsonValue::Array(vec![]));
    }

    #[test]
    fn test_parse_array_with_elements() {
        let result = parse_json("[1, 2, 3]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Integer(1),
                JsonValue::Integer(2),
                JsonValue::Integer(3),
            ])
        );
    }

    #[test]
    fn test_parse_mixed_array() {
        let result = parse_json("[\"hello\", 42, [1, 2]]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::String("hello".to_string()),
                JsonValue::Integer(42),
                JsonValue::Array(vec![JsonValue::Integer(1), JsonValue::Integer(2),]),
            ])
        );
    }

    #[test]
    fn test_parse_empty_object() {
        let result = parse_json("{}").unwrap();
        assert_eq!(result, JsonValue::Object(HashMap::new()));
    }

    #[test]
    fn test_parse_simple_object() {
        let result = parse_json("{\"name\": \"John\", \"age\": 30}").unwrap();
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), JsonValue::String("John".to_string()));
        expected.insert("age".to_string(), JsonValue::Integer(30));
        assert_eq!(result, JsonValue::Object(expected));
    }

    #[test]
    fn test_parse_nested_object() {
        let result = parse_json("{\"person\": {\"name\": \"John\", \"age\": 30}}").unwrap();
        let mut inner = HashMap::new();
        inner.insert("name".to_string(), JsonValue::String("John".to_string()));
        inner.insert("age".to_string(), JsonValue::Integer(30));

        let mut expected = HashMap::new();
        expected.insert("person".to_string(), JsonValue::Object(inner));
        assert_eq!(result, JsonValue::Object(expected));
    }

    #[test]
    fn test_parse_object_with_array() {
        let result = parse_json("{\"numbers\": [1, 2, 3]}").unwrap();
        let mut expected = HashMap::new();
        expected.insert(
            "numbers".to_string(),
            JsonValue::Array(vec![
                JsonValue::Integer(1),
                JsonValue::Integer(2),
                JsonValue::Integer(3),
            ]),
        );
        assert_eq!(result, JsonValue::Object(expected));
    }

    #[test]
    fn test_parse_with_whitespace() {
        let result = parse_json("  {  \"name\"  :  \"John\"  ,  \"age\"  :  30  }  ").unwrap();
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), JsonValue::String("John".to_string()));
        expected.insert("age".to_string(), JsonValue::Integer(30));
        assert_eq!(result, JsonValue::Object(expected));
    }

    #[test]
    fn test_parse_invalid_json() {
        assert!(parse_json("{\"name\": }").is_err());
        assert!(parse_json("[1, 2,]").is_err());
        assert!(parse_json("\"unterminated string").is_err());
        assert!(parse_json("{name: \"John\"}").is_err()); // Unquoted key
    }

    #[test]
    fn test_parse_array_and_object_unexpected_end() {
        assert!(parse_json("[").is_err());
        assert!(parse_json("{").is_err());
    }

    #[test]
    fn test_parse_number_invalid_after_minus() {
        assert!(parse_json("-").is_err());
        assert!(parse_json("-x").is_err());
    }

    #[test]
    fn test_parse_unexpected_top_level_char() {
        assert!(parse_json("t").is_err());
        assert!(parse_json(":").is_err());
    }

    #[test]
    fn test_parse_object_key_not_string_and_missing_colon() {
        assert!(parse_json("{123: 1}").is_err());
        assert!(parse_json("{\"a\" 1}").is_err());
    }
}
