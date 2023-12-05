# OEIS

Ever needed to search the [OEIS](https://oeis.org/A000924) from the command line? Yeah... me neither.

## Examples

You can use the CLI interactively:

<!--
ffmpeg -i demo.mov -loop 0 -vf fps=10 demo.gif
-->

![oeis](https://github.com/alipatti/oeis.rs/assets/78563685/a3e41454-a945-497a-979a-68d1cb92ef47)

Or non-interactively:

```bash
# Search for sequences on the OEIS website
❯ oeis 2 3 5 7 --online

# Go directly to the OEIS page for my favorite sequence
❯ oeis 1 1 1 1 --lucky
```

## Installation

```bash
# download
❯ git clone https://github.com/alipatti/oeis.git

# install
❯ cd oeis && cargo install --path .
```

At the moment, you'll need a recent rust compiler. I'll distribute binaries if there's enough demand (...but I have a sneaking suspicion that there never will be).
