extern crate ers;

use std::io::BufferedWriter;
use std::io::fs::File;
use ers::blocks::{Text,Header,Declaration,Code,Print,Block,Pos};
use ers::Template;

#[test]
fn test_template_new() {
  let some_blocks = ~[];
  let template = Template::new(~"some_path.ers", some_blocks);

  assert!(~[] == template.blocks);

}

#[test]
fn test_template_write() {
  let path = "build/foo.rs";
  let mut out_writer = BufferedWriter::new(~File::create(&Path::new(path)).unwrap() as ~Writer);

  let tmpl:Template =
    Template::new(~"foo.ers",~[
                  ~Block{class: Text,        content: ~"<html>",       pos: Pos{line_no: 4}},
                  ~Block{class: Header,      content: ~"use std::io;", pos: Pos{line_no: 1}},
                  ~Block{class: Declaration, content: ~"pub fn Template(writer: &mut Writer, i : int)", pos: Pos{line_no: 2}},
                  ~Block{class: Code,        content: ~"for n in range(0, i - 1) {", pos: Pos{line_no: 3}},
                  ~Block{class: Text,        content: ~"<p class=\"foobar\">",          pos: Pos{line_no: 5}},
                  ~Block{class: Print,       content: ~"n + 1",        pos: Pos{line_no: 6}},
                  ~Block{class: Text,        content: ~"</p>",         pos: Pos{line_no: 7}},
                  ~Block{class: Code,        content: ~"}",            pos: Pos{line_no: 8}},
                  ~Block{class: Text,        content: ~"</html>",      pos: Pos{line_no: 9}}
                  ]);
  tmpl.write_formatted(&mut out_writer);

  out_writer.flush();
}
