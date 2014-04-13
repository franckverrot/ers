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

use std::io::fs::File;
use std::io::BufferedReader;
use scanner::Scanner;
use template::Template;
use blocks::Block;

mod scanner;
mod blocks;

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

