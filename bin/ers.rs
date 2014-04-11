extern crate ers;

use std::os;
use std::io::BufferedWriter;
use std::io::fs::File;
use ers::{Parser};

fn main() {
  let args = os::args();

  match args.len() {
    3 => {
      let template = Parser::new().parse_path(args[1].clone());

      match template {
        Some(template) => {
          let mut out_writer = BufferedWriter::new(~File::create(&Path::new(args[2].clone())).unwrap() as ~Writer);

          template.write_formatted(&mut out_writer);

          out_writer.flush();
        },
        _ => fail!("Something went wrong !")
      }
    },
    _ => {
      println!("
ers Copyright (C) 2014 Franck Verrot <franck@verrot.fr>
This program comes with ABSOLUTELY NO WARRANTY; for details type `open LICENSE.txt'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `make license' for details.

 λ ers <input-file> <output-file>

 e.g: bin/ers my-file.ers my-file-template.rs");
    }
  }

}
