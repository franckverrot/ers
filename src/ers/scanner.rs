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

use std::iter::Peekable;
use blocks::{Block, Pos};

mod blocks;

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
        let s = ::std::str::from_char(c);
        if ::std::str::eq(&s,&~"<") {
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
        return Some(Block{class: ::blocks::Text, content: ~"<", pos: Pos{line_no: self.current_line }});
      },
      Some(c) => {
        let s = ::std::str::from_char(c);

        if ::std::str::eq(&s,&~"%") {
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
        let s = ::std::str::from_char(c);
        if ::std::str::eq(&s,&~"!") {
          return self.scanDeclarationBlock();
        }
        else if ::std::str::eq(&s,&~"%") {
          return self.scanHeaderBlock();
        }
        else if ::std::str::eq(&s,&~"=") {
          return self.scanPrintBlock();
        } else {
          match self.scanContent(~"") {
            None    => None,
            Some(ctn) => Some(Block{class: ::blocks::Code, content: format!("{:s}{:s}", s, ctn), pos: Pos{line_no: self.current_line}})
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
    return Some(Block{class: ::blocks::Declaration, content: output, pos: Pos{line_no: self.current_line}});
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
    return Some(Block{class: ::blocks::Header, content: output, pos: Pos{line_no: self.current_line}});
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
    return Some(Block{class: ::blocks::Print, content: output, pos: Pos{line_no: self.current_line}});
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
          let s = ::std::str::from_char(c);
          if ::std::str::eq(&s,&~"<") {
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
    return Some(Block{class: ::blocks::Text, content: output, pos: Pos{line_no: self.current_line}});
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
        let s = ::std::str::from_char(val);
        output.push_str(s);

        loop {
          let c = self.read_char();
          match c {
            None => {
              return None;
            },
            Some(c) => {
              let s = ::std::str::from_char(c);
              if ::std::str::eq(&s,&~"%") {
                let inner_c = self.read_char();
                match inner_c {
                  None => { return None; },
                  Some(inner_c) => {
                    let inner_s = ::std::str::from_char(inner_c);
                    if ::std::str::eq(&inner_s,&~">") {
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
        let s = ::std::str::from_char(val);
        output.push_str(s);

        loop {
          let c = self.read_char();
          match c {
            None => { return None; },
            Some(c) => {
              let s = ::std::str::from_char(c);
              if ::std::str::eq(&s,&~"%") {
                let inner_c = self.read_char();
                match inner_c {
                  None => { return None; },
                  Some(inner_c) => {

                    let s = ::std::str::from_char(c);
                    if ::std::str::eq(&s,&~"%") {
                      let inner_c = self.read_char();
                      match inner_c {
                        None => { return None; },
                        Some(inner_c) => {
                          let inner_s = ::std::str::from_char(inner_c);
                          if ::std::str::eq(&inner_s,&~">") {
                            break;
                          } else if ::std::str::eq(&inner_s,&~"\n") {
                            return None;
                          } else {
                            output.push_char('%');
                            output.push_char('%');
                            output.push_char(inner_c);
                          }
                        }
                      }
                    } else if ::std::str::eq(&s,&~"\n") {
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
