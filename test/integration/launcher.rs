extern crate foo;
use std::io::stdio::stdout;
use std::io::BufferedWriter;
use foo::Template;

#[test]
fn test_execution_of_produce_file() {
  foo::Template(&mut BufferedWriter::new(~stdout() as ~Writer), 4);
}

#[test]
fn test_parsing_and_execution() {
}
