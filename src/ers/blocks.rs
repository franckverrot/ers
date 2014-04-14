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

/**
`Class` is the base type of a `Block`.
*/
#[deriving(Eq,Clone)]
pub enum Class {
  /// Holds the function name that will represent the final template function
  Declaration,

  /// Defines the dependencies of the final function
  Header,

  /// Represents Rust code to be executed
  Code,

  /// Represents simple portions of text
  Text,

  /// Will output the result of the execution of the Rust code it holds
  Print
}

/**
Pos
*/
#[deriving(Eq, Clone)]
pub struct Pos {
  /// Internal line number
  line_no: int
}

impl Pos {
  /**
    `write` will write the `Pos` content to the `writer`
    */
  #[allow(unused_must_use)]
  pub fn write(&self, writer:&mut Writer) {
    writer.write_line(format!("\n//line {:d}", self.line_no));
  }
}

/**
Block
*/
#[deriving(Eq,Clone)]
pub struct Block {
  /// Block's "class" (header, declaration, etc.)
  class: Class,

  /// Block's data
  content: ~str,

  /// Block's position in the template
  pos: Pos
}

impl Block {
  /**
    `write` will write the block's content to the `writer`
    */
  #[allow(unused_must_use)]
  pub fn write(&self, writer:&mut Writer) {
    let mut w = writer;
    self.pos.write(&mut w);

    let content = self.content.escape_default();
    match self.class {
      Header      => {
        w.write_line(content);
      },
      Declaration => {
        w.write_str(content);
        w.write_str(" {\n");
      },
      Text => {
        w.write_str(format!(
            "writer.write_line(\"{:s}\");\n",
            content
            ));
      },
      Print => {
        w.write_str("writer.write_line(format!(\"{:?}\", ");
        w.write_str(format!("{:s}", content));
        w.write_str("));\n");
      },
      _ => {
        w.write_line(content);
      }
    }
  }
}
