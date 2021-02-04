# sidbang64

2020/2021 - w4rp8

install:
- rust toolchain from https://www.rust-lang.org/tools/install
- it's currently required to downgrade rust to 1.47 to produce a stable build:
  `rustup install 1.47.0`
  `rustup default 1.47.0`

build:
- cargo build --release

have fun:
- ./target/release/sidbang64


## changelog:

0.6.1:
- set active note via keyboard input

0.6.0:
- initial public release with one demotrack