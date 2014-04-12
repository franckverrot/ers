#![crate_id = "github.com/franckverrot/ers"]
#![desc = "ers - ERb-like templating for Rust"]
#![license = "GPLv3"]
#![crate_type = "lib"]
use std::str::from_char;
use std::iter::Peekable;
use std::io::BufferedReader;
use std::io::fs::File;

#[deriving(Eq)]
pub enum Class {
  Header,
  Code,
  Text,
  Print,
  Declaration
}

pub struct Template {
  path:   ~str,
  blocks: ~[~Block]
}

#[deriving(Eq)]
pub struct Block {
  class: Class,
  content: ~str,
  pos: Pos
}

#[deriving(Eq)]
pub struct Pos {
  line_no: int
}

impl Pos {
  pub fn write(&self, writer:&mut Writer) {
    writer.write_line(format!("\n//line {:d}", self.line_no));
  }
}

/**
TemplateWriteError
**/
pub enum TemplateWriteError {
  DeclarationNotFound
}

impl Template {
  pub fn write_formatted(&self, writer: &mut Writer) -> Result<int, TemplateWriteError> {
    let mut w = writer;
    let mut blocks = 0;

    // Write headers
    let mut headers = self.blocks.iter().
      filter(|&x|
             match x.class {
               Header => { return true },
               _      => { return false }
             }
            );

    // Write Declaration
    let mut declarations = self.blocks.iter().
      filter(|&x|
             match x.class {
               Declaration => { return true },
               _           => { return false }
             }
            );

    // Write Declaration
    let mut allOtherBlocks = self.blocks.iter().
      filter(|&x|
             match x.class {
               Header | Declaration => { return false },
               _           => { return true }
             }
            );

    for block in headers        { blocks+=1; block.write(&mut w); }
    for block in declarations   { blocks+=1; block.write(&mut w); }
    for block in allOtherBlocks { blocks+=1; block.write(&mut w); }

    w.write_line("writer.flush();");
    w.write_line("}\n");
    Ok(blocks)
  }

  pub fn new(obj_path: ~str, obj_blocks: ~[~Block]) -> Template {
    return Template{path: obj_path,blocks: obj_blocks};
  }
}

impl Block {
  fn write(&self, writer:&mut Writer) {
    let mut w = writer;
    self.pos.write(&mut w);
    match self.class {
      Header      => {
        w.write_line(self.content);
      },
      Declaration => {
        w.write_str(self.content);
        w.write_str(" {\n");
      },
      Text => {
        w.write_str(format!(
            "writer.write_line(\"{:s}\");\n",
            self.content
            ));
      },
      Print => {
        w.write_str("writer.write_line(format!(\"{:?}\", ");
        w.write_str(format!("{:s}", self.content));
        w.write_str("));\n");
      },
      _ => {
        w.write_line(self.content);
      }
    }
  }
}

pub struct Parser {
  version: int
}

impl Parser {
  pub fn new() -> Parser {
    return Parser{version: 1}
  }

  pub fn parse_path(&self, path: ~str) -> Option<Template> {
    let mut blocks : ~[~Block] = ~[];

    let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
    let input    = buf.read_to_str().unwrap();
    let peekable = input.chars();

    let mut scanner = Scanner::new(peekable, path.clone());

    loop {
      match scanner.scan() {
        None => break,
        Some(block) => blocks.push(~block)
      }
    }
    Some(Template{path: path.clone(), blocks: blocks})
  }
}


pub struct Scanner<Stream> {
  buffer:  Peekable<char, Stream>,
  path:    ~str,
  pos:     Pos,
  priv current_line: int,
}

impl <Stream : Iterator<char>> Scanner<Stream> {
  pub fn new(input: Stream, path: ~str) -> Scanner<Stream> {
    Scanner {
      buffer: input.peekable(),
      path: path.clone(),
      pos: Pos{line_no: 1},
      current_line: 1
    }
  }

  pub fn read_char(&mut self) -> Option<char> {
    match self.buffer.next() {
      None => None,
      Some(character) => {
        if(character == '\n') { self.current_line += 1; }
        Some(character)
      }
    }
  }

  pub fn peek_char(&mut self) -> Option<char> {
    match self.buffer.peek() {
      None    => None,
      Some(c) => { Some(*c) }
    }
  }

  pub fn scan(&mut self) -> Option<Block> {
    let c = self.read_char();

    match c {
      None => { None },
      Some(c) => {
        let s = std::str::from_char(c);
        if std::str::eq(&s,&~"<") {
          return self.scanBlock()
        }
        else {
          return self.scanTextBlock(s)
        }
      }
    }
  }

  pub fn scanBlock(&mut self) -> Option<Block> {
    let c = self.read_char();

    match c {
      None => {
        return Some(Block{class: Text, content: ~"<", pos: Pos{line_no: self.current_line }});
      },
      Some(c) => {
        let s = std::str::from_char(c);

        if std::str::eq(&s,&~"%") {
          return self.scanCodeBlock();
        }
        else {
          return self.scanTextBlock(format!("<{:s}", s));
        }
      }
    }
  }

  pub fn scanCodeBlock(&mut self) -> Option<Block> {
    let c = self.read_char();

    match c {
      None => { None },
      Some(c) => {
        let s = std::str::from_char(c);
        if std::str::eq(&s,&~"!") {
          return self.scanDeclarationBlock();
        }
        else if std::str::eq(&s,&~"%") {
          return self.scanHeaderBlock();
        }
        else if std::str::eq(&s,&~"=") {
          return self.scanPrintBlock();
        } else {
          match self.scanContent(~"") {
            None    => None,
            Some(ctn) => Some(Block{class: Code, content: format!("{:s}{:s}", s, ctn), pos: Pos{line_no: self.current_line}})
          }
        }
      }
    }
  }

  pub fn scanDeclarationBlock(&mut self) -> Option<Block> {
    let mut output = ~"";
    match self.scanContent(~"") {
      None => { return None; },
      Some(s) => { output.push_str(s); }
    }
    return Some(Block{class: Declaration, content: output, pos: Pos{line_no: self.current_line}});
  }

  pub fn scanHeaderBlock(&mut self) -> Option<Block> {
    let mut output = ~"";
    match self.scanHeaderContent(~"") {
      None => { return None; },
      Some(s) => { output.push_str(s); }
    }
    return Some(Block{class: Header, content: output, pos: Pos{line_no: self.current_line}});
  }

  pub fn scanPrintBlock(&mut self) -> Option<Block> {
    let mut output = ~"";
    match self.scanContent(~"") {
      None => { return None; },
      Some(s) => { output.push_str(s); }
    }
    return Some(Block{class: Print, content: output, pos: Pos{line_no: self.current_line}});
  }

  pub fn scanTextBlock(&mut self, s: ~str) -> Option<Block> {
    let mut output = s.clone();
    loop {
      let c = self.peek_char();
      match c {
        None => {
          break;
        },
        Some(c) => {
          let s = std::str::from_char(c);
          if std::str::eq(&s,&~"<") {
            //self.buffer.push_bytes(&mut b,1);
            break;
          }
          else {
            let c = self.read_char().unwrap();
            output.push_char(c);
          }
        }
      }
    }
    return Some(Block{class: Text, content: output, pos: Pos{line_no: self.current_line}});
  }

  pub fn scanContent(&mut self, prefix: ~str) -> Option<~str> {
    let mut output = prefix.clone();

    match self.read_char() {
      None => {
        return None;
      },
      Some(val) => {
        let s = std::str::from_char(val);
        output.push_str(s);

        loop {
          let c = self.read_char();
          match c {
            None => {
              return None;
            },
            Some(c) => {
              let s = std::str::from_char(c);
              if std::str::eq(&s,&~"%") {
                let inner_c = self.read_char();
                match inner_c {
                  None => { return None; },
                  Some(inner_c) => {
                    let inner_s = std::str::from_char(inner_c);
                    if std::str::eq(&inner_s,&~">") {
                      break;
                    }
                    else {
                      output.push_char('%');
                      output.push_char(inner_c);
                    }
                  }
                }
              }
              else {
                output.push_char(c);
              }
            }
          }
        }
        return Some(output);
      }
    }
  }

  pub fn scanHeaderContent(&mut self, prefix: ~str) -> Option<~str> {
    let mut output = prefix.clone();

    match self.read_char() {
      None => { return None; },
      Some(val) => {
        let s = std::str::from_char(val);
        output.push_str(s);

        loop {
          let c = self.read_char();
          match c {
            None => { return None; },
            Some(c) => {
              let s = std::str::from_char(c);
              if std::str::eq(&s,&~"%") {
                let inner_c = self.read_char();
                match inner_c {
                  None => { return None; },
                  Some(inner_c) => {

                    let s = std::str::from_char(c);
                    if std::str::eq(&s,&~"%") {
                      let inner_c = self.read_char();
                      match inner_c {
                        None => { return None; },
                        Some(inner_c) => {
                          let inner_s = std::str::from_char(inner_c);
                          if std::str::eq(&inner_s,&~">") {
                            break;
                          } else if std::str::eq(&inner_s,&~"\n") {
                            return None;
                          } else {
                            output.push_char('%');
                            output.push_char('%');
                            output.push_char(inner_c);
                          }
                        }
                      }
                    } else if std::str::eq(&s,&~"\n") {
                      return None;
                    }
                    else {
                      output.push_char('%');
                      output.push_char(inner_c);
                    }
                  }
                }
              }
              else {
                output.push_char(c);
              }
            }
          }
        }
        return Some(output);
      }
    }
  }
}
