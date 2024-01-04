#[derive(Debug)]
pub enum Event {
    StartElement(String),
    EndElement(String),
    TextContent(String),
    Attribute(String, String),
}

pub struct Lexer {
    content: Vec<char>,
    tag_stack: Vec<String>,
    events: Vec<Event>,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content: content.chars().collect::<Vec<char>>(),
            tag_stack: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> &[Event] {
        while !self.content.is_empty() {
            self.take_whitespaces();

            if self.content.is_empty() {
                break;
            }

            // TODO: Implement html declaration eg, `<!Doctype html>`.
            // TODO: Implement html comment.
            // TODO: Implement not closing tag eg, `<meta ...>`

            // End element
            if self.content.len() > 1 && self.content[0] == '<' && self.content[1] == '/' {
                self.take_end_element();
            }

            // Start element
            if !self.content.is_empty() && self.content[0] == '<' {
                self.take_start_element();
            }

            // Text content
            if !self.content.is_empty() {
                self.take_text_content();
            }
        }

        &self.events
    }

    // `</TAG_NAME>``
    fn take_end_element(&mut self) {
        let tag_name = self.take_tag_name(2);
        self.events.push(Event::EndElement(tag_name.clone()));

        // End element validation
        if self.tag_stack.last() == Some(&tag_name) {
            self.tag_stack.pop();
        } else {
            eprintln!("ERROR: Invalid closing tag `{tag_name}`.")
        }

        self.take_whitespaces();

        if self.content[0] == '>' {
            self.get_slice(0, 1);
        } else {
            eprintln!("ERROR: Invalid closing tag with extra args.");
        }
    }

    // `<TAG_NAME ATTRIBUTES>` | `<TAG_NAME ATTRIBUTES/>`
    fn take_start_element(&mut self) {
        let tag_name = self.take_tag_name(1);
        self.events.push(Event::StartElement(tag_name.clone()));
        self.tag_stack.push(tag_name);

        self.take_attributes();
        self.take_whitespaces();

        // SELF_END_ELEMENT `/>`
        if self.content.len() > 1 && self.content[0] == '/' && self.content[1] == '>' {
            self.get_slice(0, 2);

            if let Some(last_tag) = self.tag_stack.pop() {
                self.events.push(Event::EndElement(last_tag));
            } else {
                eprintln!("ERROR: there is no tag.");
            }
        }
        // EXPECTED `>`
        else if self.content[0] == '>' {
            self.get_slice(0, 1);
        } else {
            eprintln!("ERROR: expected `>` on start element.");
        }
    }

    fn take_tag_name(&mut self, start: usize) -> String {
        self.take_while_from(start, |x| x.is_alphabetic() || x.is_alphanumeric())
    }

    fn take_text_content(&mut self) {
        let value = self.take_while(|x| x != '<');
        let value = value.replace("\n", "");
        let value = value.replace("\t", "");
        let value = value.trim().to_string();

        if value.is_empty() {
            return;
        }

        self.events.push(Event::TextContent(value));
    }

    fn take_attributes(&mut self) {
        // after start a element, until we go to '>' | '/', means we are collecting attributes
        // eg, `<tag key=value key=value>` || `<tag key=value />`;
        while (self.content[0] != '>') && (self.content[0] != '/') {
            self.take_attribute()
        }
    }

    // `KEY=VALUE` || `KEY`
    fn take_attribute(&mut self) {
        self.take_whitespaces();

        let key =
            self.take_while(|x| x.is_alphabetic() || x.is_alphanumeric() || x == '-' || x == '_');

        let value = self.take_attribute_value();

        self.events.push(Event::Attribute(key, value));

        self.take_whitespaces();
    }

    fn take_attribute_value(&mut self) -> String {
        if self.content.is_empty() || self.content[0] != '=' {
            return String::from("");
        }

        self.get_slice(0, 1);

        let mut qoute_count = 0;

        // TODO: Implement string logic.
        let value = self.take_while(|x| {
            // String identifire
            if x == '"' {
                qoute_count += 1;
                return if qoute_count == 2 { false } else { true };
            }

            // If value is a string
            if qoute_count == 1 {
                return true;
            }

            x != ' ' || x != '>' || x != '/'
        });

        if qoute_count == 0 {
            return value;
        }

        // when qoute == 2 THEN we break, means we don't count ending '"'
        // we need to clean up that '"'
        self.get_slice(0, 1);

        value[1..].to_string()
    }

    fn take_whitespaces(&mut self) {
        self.take_while(|x| x.is_whitespace());
    }

    fn take_while<F>(&mut self, predict: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        self.take_while_from(0, predict)
    }

    fn take_while_from<F>(&mut self, start: usize, mut predict: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut i = start;

        while self.content.len() > i && predict(self.content[i]) {
            i += 1;
        }

        self.get_slice(start, i)
    }

    fn get_slice(&mut self, from: usize, to: usize) -> String {
        let value = self.content[from..to].iter().collect::<String>();
        self.content = self.content[to..].to_vec();

        value
    }
}
