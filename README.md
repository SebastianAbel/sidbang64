# sidbang64

2020/2021 - w4rp8

about:
Sidbang64 is a loop-sequencer for sid-chip multispeed sounds.

Most sid-music is made with an update rate once per screen refresh of the video system (50Hz/60Hz). Multispeed means that state updates of the player happen multiple times during a screen refresh, eg. 16x translates to 800Hz on a PAL system (actually it's 16 x 50.12454212Hz, so somewhat above 800Hz).  
Instruments are configured by setting up oscillators for different parameters. The "sample frequency" of the oscillators is determined by the set multispeed factor. The interface reflects this by displaying an oscillator frequency (f) together with the update ticks (t).

Songs can be exported to be played on c64. Currently the export format is essentially a dump of sid-state changes to keep the replayer fast and simple. By this the songlength is determined by the complexity of the used instruments and might overflow the memory limit quite fast.
This will be addressed until version 1.0 by porting some of the instrument logic directly into the replayer.

For further instructions refer to the [./doc/sidbang_ui.pdf](./doc/sidbang_ui.pdf)

source:
- https://github.com/SebastianAbel/sidbang64

install:
- rust toolchain from https://www.rust-lang.org/tools/install
- it's currently required to downgrade rust to 1.47 to produce a stable build:\
  `rustup install 1.47.0`\
  `rustup default 1.47.0`

build:
- `cargo build --release`

run:
- `./target/release/sidbang64`

show options:
- `./target/release/sidbang64 --help`

run with session, and play it:
- `./target/release/sidbang64 --autoload --autoplay --session monomelvin`

examples:
- some projects are stored in the `/bng` subfolder
- type in one of the names from there, load and press space to play
- switch from loop- to songmode if needed


## history:

0.6.4
- 64 filterpatches to store settings
- disabled 1x-3x speeds for now as these seem not to play as intended

0.6.3
- fixed bug in exporter
- small fixes in ui
- slightly optimized c64 replayer code with dynamic filter sequence

0.6.2:
- insert/delete and copy/paste for entries in the loop-sequence
- moved copy/paste button for pattern below the loop-sequence
- added command line arguments
- disabled the default vsync

0.6.1:
- set active note via keyboard input

0.6.0:
- initial public release with one demotrack