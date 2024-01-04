# Html Parser Rs

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

`html-parser-rs` is a Rust crate for parsing HTML documents. It supports reading HTML documents in an event-based fashion. This allows you to receive events for different elements, attributes, and text content during the parsing process.

## Features

- **Event-Based Parsing:** Receive events for different elements, attributes, and text content during parsing.

## Usage

Add this crate to your `Cargo.toml` file:

```toml
[dependencies]
html-parser-rs = "0.1.0"
```


```rs
use html_parser_rs::Lexer;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./examples/index.html")
        .expect("Failed to read file.");

    let mut lexer = Lexer::new(contents);

    for token in lexer.parse() {
        println!("{token:?}");
    }
}
```

## Events

- `StartElement(TAG_NAME)`: Triggered when an HTML element starts.

- `EndElement(TAG_NAME)`: Triggered when an HTML element ends.

- `TextContent(TEXT)`: Triggered when text content is encountered.

- `Attribute(NAME, VALUE)`: Triggered when an attribute is encountered.

## Contributing

Contributions are welcome! I would like you to contribute in this project.

## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](/issues) section for a list of features, enhancements, and bug fixes that are planned.

## License

This project is licensed under the MIT License - see the [LICENSE](/LICENSE.md) file for details.
