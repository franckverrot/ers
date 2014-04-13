/**
`Class` is the base type of a `Block`.
*/
#[deriving(Eq)]
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
#[deriving(Eq)]
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
#[deriving(Eq)]
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
