# ers - ERb-like templating engine for Rust

ers is an ERb-style templating language for Rust.

ers templates will be turned into statically compiled Rust functions at your
will, allowing you to link them to your other projects.

The library infrastructure is largely inspired by `ego`.

# INSTALLATION

Using the Makefile should be enough to build the main transpiler:

    make ers

# USAGE

In order to compile a template `foo.ers` like:

```rust
<%! pub fn Template(writer: &mut Writer, i: int) %>
<%% use std::io; %%>
<body>
  <% for n in range(0, i - 1) { %>
    <p>
      <%= n + 1 %>
    </p>
  <% } %>
</body>
```

into a pure Rust function, run:

    bin/ers foo.ers foo.rs

Now that the template function is done, use the `Template` function in your code, say `my-file.rs`, like this:

```rust
extern crate foo;
use std::io::stdio::stdout;
use std::io::BufferedWriter;
use foo::Template;

fn main() {
  foo::Template(&mut BufferedWriter::new(~stdout() as ~Writer), 4);
}
```

Compiling and running the `my-file` program will output:

```html
<body>
  <p>
    1
  </p>
  <p>
    2
  </p>
  <p>
    3
  </p>
</body>
```

## TEMPLATE DEFINITION

### DECLARING THE TEMPLATE METHOD NAME

### USING RUST CODE

## EXAMPLE

# LICENSE

GPLv3. Copyright Franck Verrot - 2014.
