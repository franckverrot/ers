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

use blocks::Block;
use blocks::Header;
use blocks::Declaration;

/**
Template
*/
#[deriving(Clone)]
pub struct Template {
  /// Location of the template on the filesystem
  path:   ~str,

  /// The collection of blocks that make the template
  blocks: ~[~Block]
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

  /**
    Creates a new template from a path and an array of blocks
    */
  pub fn new(obj_path: ~str, obj_blocks: ~[~Block]) -> Template {
    return Template{path: obj_path,blocks: obj_blocks};
  }
}

/**
TemplateWriteError
*/
pub enum TemplateWriteError {
  /// Error raised by a missing declaration
  DeclarationNotFound
}

