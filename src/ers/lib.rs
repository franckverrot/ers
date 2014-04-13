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

pub use template::Template;
pub use blocks::{Block, Class, Pos};
pub use parser::Parser;
pub use scanner::Scanner;

/// Template implementation
pub mod template;

/// Template blocks definitions and implementations
pub mod blocks;

/// High-level parser
pub mod parser;

/// Template scanner
pub mod scanner;
