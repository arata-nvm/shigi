use crate::dom;
use std::collections::HashMap;

pub fn parse(source: String) -> dom::Document {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    let root_node = if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    };

    dom::Document::new(root_node)
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();

            if self.starts_with("<!--") {
                self.consume_comment();
                continue;
            }

            if self.starts_with("<!") {
                self.consume_doctype();
                continue;
            }

            if self.eof() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node());
        }
        return nodes;
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_element(&mut self) -> dom::Node {
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();

        if self.starts_with("/>") {
            assert_eq!(self.consume_char(), '/');
            assert_eq!(self.consume_char(), '>');
            return dom::elem(tag_name, attrs, Vec::new());
        }

        assert_eq!(self.consume_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        return dom::elem(tag_name, attrs, children);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.starts_with("/>") || self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');

        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        return value;
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
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
            if self.starts_with("-->") {
                assert_eq!(self.consume_char(), '-');
                assert_eq!(self.consume_char(), '-');
                assert_eq!(self.consume_char(), '>');
                return;
            }
            self.consume_char();
        }
    }

    fn consume_doctype(&mut self) {
        self.consume_while(|c| c != '>');
        assert_eq!(self.consume_char(), '>');
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

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::dom::{elem, text};
    use std::collections::HashMap;

    #[test]
    fn test_parse() {
        let html_source = r#"
        <!DOCTYPE html>
        <html>
            <!-- This is a comment -->
            <body>
                <h1>Title</h1>
                <div id="main" class="test">
                    <p>Hello <em>world</em>!</p>
                     <img src="something.png" alt="Something" width="100" height="200" />
                </div>
            </body>
        </html>"#
            .to_string();

        let expected = elem(
            "html".to_string(),
            HashMap::new(),
            vec![elem(
                "body".to_string(),
                HashMap::new(),
                vec![
                    elem(
                        "h1".to_string(),
                        HashMap::new(),
                        vec![text("Title".to_string())],
                    ),
                    elem(
                        "div".to_string(),
                        {
                            let mut map = HashMap::new();
                            map.insert("id".to_string(), "main".to_string());
                            map.insert("class".to_string(), "test".to_string());
                            map
                        },
                        vec![
                            elem(
                                "p".to_string(),
                                HashMap::new(),
                                vec![
                                    text("Hello ".to_string()),
                                    elem(
                                        "em".to_string(),
                                        HashMap::new(),
                                        vec![text("world".to_string())],
                                    ),
                                    text("!".to_string()),
                                ],
                            ),
                            elem(
                                "img".to_string(),
                                {
                                    let mut map = HashMap::new();
                                    map.insert("src".to_string(), "something.png".to_string());
                                    map.insert("alt".to_string(), "Something".to_string());
                                    map.insert("width".to_string(), "100".to_string());
                                    map.insert("height".to_string(), "200".to_string());
                                    map
                                },
                                vec![],
                            ),
                        ],
                    ),
                ],
            )],
        );

        let actual = parse(html_source).root_node;

        assert_eq!(expected, actual);
    }
}
