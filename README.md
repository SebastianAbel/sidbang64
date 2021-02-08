# sidbang64

2020/2021 - w4rp8

install:
- rust toolchain from https://www.rust-lang.org/tools/install
- it's currently required to downgrade rust to 1.47 to produce a stable build:\
  `rustup install 1.47.0`\
  `rustup default 1.47.0`

build:
- `cargo build --release`

have fun:
- `./target/release/sidbang64`

show options:
- `./target/release/sidbang64 --help`

## history:

0.6.2:
- insert/delete and copy/paste for entries in the loop-sequence
- moved copy/paste button for pattern below the loop-sequence
- added command line arguments
- disabled the default vsync

0.6.1:
- set active note via keyboard input

0.6.0:
- initial public release with one demotrack