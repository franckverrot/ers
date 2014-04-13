// ers - ERb-like template engine
// Copyright (C) 2014 Franck Verrot <franck@verrot.fr>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

/*!

# ers is an ERb-style templating language for Rust.

ers templates will be turned into statically compiled Rust functions at your
will, allowing you to link them to your other projects.

Copyright (C) 2014 Franck Verrot <franck@verrot.fr>

Source code: https://github.com/franckverrot/ers
*/
#![crate_id = "github.com/franckverrot/ers"]
#![desc = "ers - ERb-like templating for Rust"]
#![license = "GPLv3"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![deny(missing_doc)]
use std::str::from_char;
use std::iter::Peekable;
use std::io::BufferedReader;
use std::io::fs::File;
pub use blocks::{Block, Class, Pos};

/// Template blocks definitions and implementations
pub mod blocks;

/**
Template
*/
pub struct Template {
  /// Location of the template on the filesystem
  path:   ~str,

  /// The collection of blocks that make the template
  blocks: ~[~Block]
}

/**
TemplateWriteError
*/
pub enum TemplateWriteError {
  /// Error raised by a missing declaration
  DeclarationNotFound
}

impl Template {
  /**
    `write_formatted` will write the `Template` content to the `writer`
    */
  #[allow(unused_must_use)]
  pub fn write_formatted(&self, writer: &mut Writer) -> Result<int, TemplateWriteError> {
    let mut w = writer;
    let mut blocks = 0;

    // Write headers
    let mut headers = self.blocks.iter().
      filter(|&x|
             match x.class {
               blocks::Header => { return true },
               _      => { return false }
             }
            );

    // Write Declaration
    let mut declarations = self.blocks.iter().
      filter(|&x|
             match x.class {
               blocks::Declaration => { return true },
               _           => { return false }
             }
            );

    // Write Declaration
    let mut allOtherBlocks = self.blocks.iter().
      filter(|&x|
             match x.class {
               blocks::Header | blocks::Declaration => { return false },
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

  /**
    Creates a new template from a path and an array of blocks
    */
  pub fn new(obj_path: ~str, obj_blocks: ~[~Block]) -> Template {
    return Template{path: obj_path,blocks: obj_blocks};
  }
}

/**
Parser
*/
pub struct Parser {
  /// The version of the parser. This will prevent accidental regressions if
  /// the grammar of Ers change in the future
  version: int
}

impl Parser {
  /**
    Creates a new Parser
    */
  pub fn new() -> Parser {
    return Parser{version: 1}
  }

  /**
    `parse_path` is the parser main function that returns an `Option<Template>`
    */
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

/**
Scanner
*/
pub struct Scanner<Stream> {
  /// Internal buffer used to parse the content of the buffer
  buffer:  Peekable<char, Stream>,

  /// Location of the template to be parsed on the local filesystem
  path:    ~str,

  /// Last block position parsed by the scanner
  pos:     Pos,

  /// Last line parsed by the scanner
  priv current_line: int,
}

impl <Stream : Iterator<char>> Scanner<Stream> {
  /**
    `new` build a `Scanner` object from a `Stream` and a `path`
   */
  pub fn new(input: Stream, path: ~str) -> Scanner<Stream> {
    Scanner {
      buffer: input.peekable(),
      path: path.clone(),
      pos: Pos{line_no: 1},
      current_line: 1
    }
  }

  /**
    `read_char` is an utility method that consumes the next character
    to be read
   */
  pub fn read_char(&mut self) -> Option<char> {
    match self.buffer.next() {
      None => None,
      Some(character) => {
        if character == '\n' { self.current_line += 1; }
        Some(character)
      }
    }
  }

  /**
    `peek_char` is an utility method that inspects the next character
    to be read
   */
  pub fn peek_char(&mut self) -> Option<char> {
    match self.buffer.peek() {
      None    => None,
      Some(c) => { Some(*c) }
    }
  }

  /**
    `scan` is the entry point of a `Scanner`
   */
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

  /**
    `scanBlock` is the entry point of the block parsing strategy
   */
  pub fn scanBlock(&mut self) -> Option<Block> {
    let c = self.read_char();

    match c {
      None => {
        return Some(Block{class: blocks::Text, content: ~"<", pos: Pos{line_no: self.current_line }});
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

  /**
    `scanCodeBlock` will parse `Code` blocks
   */
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
            Some(ctn) => Some(Block{class: blocks::Code, content: format!("{:s}{:s}", s, ctn), pos: Pos{line_no: self.current_line}})
          }
        }
      }
    }
  }

  /**
    `scanDeclarationBlock` will parse `Declaration` blocks
   */
  pub fn scanDeclarationBlock(&mut self) -> Option<Block> {
    let mut output = ~"";
    match self.scanContent(~"") {
      None => { return None; },
      Some(s) => { output.push_str(s); }
    }
    return Some(Block{class: blocks::Declaration, content: output, pos: Pos{line_no: self.current_line}});
  }

  /**
    `scanHeaderBlock` will parse `Header` blocks
   */
  pub fn scanHeaderBlock(&mut self) -> Option<Block> {
    let mut output = ~"";
    match self.scanHeaderContent(~"") {
      None => { return None; },
      Some(s) => { output.push_str(s); }
    }
    return Some(Block{class: blocks::Header, content: output, pos: Pos{line_no: self.current_line}});
  }

  /**
    `scanPrintBlock` will parse `Print` blocks
   */
  pub fn scanPrintBlock(&mut self) -> Option<Block> {
    let mut output = ~"";
    match self.scanContent(~"") {
      None => { return None; },
      Some(s) => { output.push_str(s); }
    }
    return Some(Block{class: blocks::Print, content: output, pos: Pos{line_no: self.current_line}});
  }

  /**
    `scanTextBlock` will parse `Text` blocks
   */
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
    return Some(Block{class: blocks::Text, content: output, pos: Pos{line_no: self.current_line}});
  }

  /**
    `scanContent` will parse any text section
   */
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

  /**
    `scanHeaderContent` will parse `Header` blocks
   */
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
