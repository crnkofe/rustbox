# Rustbox

This fork of rustbox is a simple-to-use Rust wrapper around [termbox2](https://github.com/crnkofe/termbox-sys).

The original implementation of this was inspired by [Aaron Pribadi](http://github.com/apribadi/rust-termbox), so big props to him for the original work.

**NOTE** This is under development, and the APIs may change as I figure out more how Rust works and as the language itself changes

## Usage

In your `Cargo.toml` add the following:

```toml
[dependencies]
rustbox = { git = "https://github.com/crnkofe/rustbox" }
```

Then, in your `src/example.rs`:

```rust
extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;
use rustbox::PressedKey;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    PressedKey::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
```

**NOTE:** this example can also be run with `cargo run --example hello-world`.

