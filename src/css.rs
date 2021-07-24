pub mod stylesheet;
pub use stylesheet::*;

pub mod default;

pub fn parse(source: String) -> Stylesheet {
    let rules = Parser {
        pos: 0,
        input: source,
    }
    .parse_rules();

    return Stylesheet { rules };
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();

            if self.starts_with("/*") {
                self.consume_comment();
                continue;
            }

            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        return selector;
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                break;
            }
            declarations.push(self.parse_declaration());
        }
        assert_eq!(self.consume_char(), '}');
        return declarations;
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_identifier();

        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();

        let value = self.parse_value();

        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        return Declaration { name, value };
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' | '-' => self.parse_length_value(),
            '#' => self.parse_color_value(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length_value(&mut self) -> Value {
        let f = self.parse_float();
        match self.next_char() {
            ';' => Value::Number(f),
            _ => Value::Length(f, self.parse_unit()),
        }
    }

    fn parse_float(&mut self) -> f32 {
        let float_literal = self.consume_while(|c| match c {
            '0'..='9' | '.' | '-' => true,
            _ => false,
        });

        return float_literal.parse().unwrap();
    }

    fn parse_unit(&mut self) -> Unit {
        let unit = self.parse_identifier();
        match unit.as_str() {
            "px" => Unit::Px,
            "em" => Unit::Em,
            _ => panic!(""),
        }
    }

    fn parse_color_value(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        return Value::ColorValue(Color {
            r: self.parse_hex(),
            g: self.parse_hex(),
            b: self.parse_hex(),
            a: 255,
        });
    }

    fn parse_hex(&mut self) -> u8 {
        let hex_literal = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        return u8::from_str_radix(hex_literal, 16).unwrap();
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn consume_comment(&mut self) {
        loop {
            if self.starts_with("*/") {
                assert_eq!(self.consume_char(), '*');
                assert_eq!(self.consume_char(), '/');
                return;
            }
            self.consume_char();
        }
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::css::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value};

    #[test]
    fn test_parse() {
        let css_source = r#"
        h1, h2, h3 { margin: auto; color: #cc0000; }
        div.note { margin-bottom: 20px; padding: 10px; }
        #answer { display: none; }"#
            .to_string();

        let expected = Stylesheet {
            rules: vec![
                Rule {
                    selectors: vec![
                        Selector::Simple(SimpleSelector {
                            tag_name: Some("h1".to_string()),
                            id: None,
                            class: vec![],
                        }),
                        Selector::Simple(SimpleSelector {
                            tag_name: Some("h2".to_string()),
                            id: None,
                            class: vec![],
                        }),
                        Selector::Simple(SimpleSelector {
                            tag_name: Some("h3".to_string()),
                            id: None,
                            class: vec![],
                        }),
                    ],
                    declarations: vec![
                        Declaration {
                            name: "margin".to_string(),
                            value: Value::Keyword("auto".to_string()),
                        },
                        Declaration {
                            name: "color".to_string(),
                            value: Value::ColorValue(Color {
                                r: 0xcc,
                                g: 0x00,
                                b: 0x00,
                                a: 0xff,
                            }),
                        },
                    ],
                },
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: Some("div".to_string()),
                        id: None,
                        class: vec!["note".to_string()],
                    })],
                    declarations: vec![
                        Declaration {
                            name: "margin-bottom".to_string(),
                            value: Value::Length(20.0, Unit::Px),
                        },
                        Declaration {
                            name: "padding".to_string(),
                            value: Value::Length(10.0, Unit::Px),
                        },
                    ],
                },
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: None,
                        id: Some("answer".to_string()),
                        class: vec![],
                    })],
                    declarations: vec![Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("none".to_string()),
                    }],
                },
            ],
        };

        let actual = parse(css_source);

        assert_eq!(expected, actual);
    }
}
