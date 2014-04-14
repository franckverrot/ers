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
      let ref bs = template.blocks;

      assert!(std::str::eq(&path.clone(), &template.path));
      assert!(bs.len()>0);

      assert!(bs[0]  == ~Block{class: Declaration, content: ~" pub fn Template(writer: &mut Writer, i: int) ", pos: Pos{line_no: 1}});
      assert!(bs[1]  == ~Block{class: Text,        content: ~"\n",                                             pos: Pos{line_no: 2}});
      assert!(bs[2]  == ~Block{class: Header,      content: ~" use std::io; ",                                 pos: Pos{line_no: 2}});
      assert!(bs[3]  == ~Block{class: Text,        content: ~"\n",                                             pos: Pos{line_no: 3}});
      assert!(bs[4]  == ~Block{class: Text,        content: ~"<html>\n",                                       pos: Pos{line_no: 4}});
      assert!(bs[5]  == ~Block{class: Code,        content: ~" for n in range(0, i - 1) { ",                   pos: Pos{line_no: 4}});
      assert!(bs[6]  == ~Block{class: Text,        content: ~"\n",                                             pos: Pos{line_no: 5}});
      assert!(bs[7]  == ~Block{class: Text,        content: ~"<p class=\"foobar",                              pos: Pos{line_no: 5}});
      assert!(bs[8]  == ~Block{class: Print,       content: ~" n ",                                            pos: Pos{line_no: 5}});
      assert!(bs[9]  == ~Block{class: Text,        content: ~"\">\n  ",                                        pos: Pos{line_no: 6}});
      assert!(bs[10] == ~Block{class: Print,       content: ~" n + 1 ",                                        pos: Pos{line_no: 6}});
      assert!(bs[11] == ~Block{class: Text,        content: ~"\n",                                             pos: Pos{line_no: 7}});
      assert!(bs[12] == ~Block{class: Text,        content: ~"</p>\n",                                         pos: Pos{line_no: 8}});
      assert!(bs[13] == ~Block{class: Code,        content: ~" } ",                                            pos: Pos{line_no: 8}});
      assert!(bs[14] == ~Block{class: Text,        content: ~"\n",                                             pos: Pos{line_no: 9}});
      assert!(bs[15] == ~Block{class: Text,        content: ~"</html>\n",                                      pos: Pos{line_no: 10}});
    }
  }
}
