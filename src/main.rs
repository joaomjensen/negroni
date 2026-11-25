use std::path::PathBuf;

use clap::{arg, command};
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;

fn main() {
    let matches = command!()
        .arg(
            arg!([PATH] "Path to the file")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .get_matches();

    if let Some(path) = matches.get_one::<PathBuf>("PATH") {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let mut parser = Parser::new();
                parser
                    .set_language(&tree_sitter_rust::LANGUAGE.into())
                    .expect("Error loading Rust grammar");

                let tree = parser.parse(&content, None).expect("Error parsing content");
                let query = Query::new(
                    &tree_sitter_rust::LANGUAGE.into(),
                    "[(line_comment) (block_comment)] @comment",
                )
                .expect("Error creating query");

                // TODO: Dogfooding

                let mut query_cursor = QueryCursor::new();
                let mut matches = query_cursor.matches(&query, tree.root_node(), content.as_bytes());

                while let Some(m) = matches.next() {
                    if let Some(capture) = m.captures.first() {
                        let comment_node = capture.node;
                        let comment_text = &content[comment_node.byte_range()];

                        if comment_text.contains("TODO") {
                            let line = comment_node.start_position().row + 1;
                            println!("Line {}: {}", line, comment_text.trim());
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", path.display(), e);
            }
        }
    }
}