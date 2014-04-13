extern crate ers;

use ers::Parser;
use ers::blocks::{Text,Header,Declaration,Code,Print,Pos,Block};

#[test]
fn test_parser_parsepath() {
  let parser = Parser::new();
  let path   = ~"test/fixtures/parsefile.ers";
  let templ  = parser.parse_path(path.clone());

  match templ {
    None => fail!("Empty template"),
    Some(template) => {

      assert!(std::str::eq(&path.clone(), &template.path));
      assert!(template.blocks.len()>0);
      assert!(
        ~[
        ~Block{class: Declaration, content: ~" pub fn Template(writer: &mut Writer, i: int) ", pos: Pos{line_no: 1}},
        ~Block{class: Text, content: ~"\n", pos: Pos{line_no: 2}},
        ~Block{class: Header, content: ~" use std::io; ", pos: Pos{line_no: 2}},
        ~Block{class: Text, content: ~"\n", pos: Pos{line_no: 3}},
        ~Block{class: Text, content: ~"<html>\n", pos: Pos{line_no: 4}},
        ~Block{class: Code, content: ~" for n in range(0, i - 1) { ", pos: Pos{line_no: 4}},
        ~Block{class: Text, content: ~"\n", pos: Pos{line_no: 5}},
        ~Block{class: Text, content: ~"<p>\n  ", pos: Pos{line_no: 6}},
        ~Block{class: Print, content: ~" n + 1 ", pos: Pos{line_no: 6}},
        ~Block{class: Text, content: ~"\n", pos: Pos{line_no: 7}},
        ~Block{class: Text, content: ~"</p>\n", pos: Pos{line_no: 8}},
        ~Block{class: Code, content: ~" } ", pos: Pos{line_no: 8}},
        ~Block{class: Text, content: ~"\n", pos: Pos{line_no: 9}},
        ~Block{class: Text, content: ~"</html>\n", pos: Pos{line_no: 10}}
        ] == template.blocks);
    }
  }
}
