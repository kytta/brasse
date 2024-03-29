# brasse

`brasse` is a [Homebrew] clone written in Rust. The goals of this project are:

- increase speeds for operations performed offline
  - `brew list` takes over 400 ms (`brasse list` takes 1 ms)
  - `brew search git` takes 869 ms
  - `brew info git` takes 530 ms
- provide good piping support to allow for, for example:
  - pipe a list of package names to `install`
  - pipe a list from `outdated` to `info`
- (optionally) provide better dependency management
  - similarly to APT, differentiate automatically and manually installed
    packages to remove the former if they have no dependents
- keep everything compatible with the official `brew` executable

## Install

### From GitHub releases

1. Open [the latest brasse release on GitHub](https://github.com/kytta/brasse/releases/latest)
2. Download the asset that corresponds to your CPU (`aarch64` being Apple Silicon (M1/M2) and `x86_64` being Intel)
3. Put the executable in your `$PATH` (for example, `~/.local/bin`)

### Build from Source

You need to have a Rust toolchain installed.

1. Clone the repo

   ```sh
   git clone https://github.com/kytta/brasse.git
   ```

2. (if you have [just](https://github.com/casey/just)) run `just install`
3. If you don't have `just`, run:

   ```sh
   cargo build --release
   install -m 755 target/release/brasse ~/.local/bin/brasse
   ```

The steps above assume `~/.local/bin` is in your `$PATH`

## Status

This is very much an experimental project.

Benchmarks are done with [`hyperfine`][hyperfine]

### Currently implemented

#### `brasse list`

Currently, this just outputs the list of formulae and casks.
Supported arguments: `-1`

| Command       |   Mean [ms] | Min [ms] | Max [ms] |       Relative |
| :------------ |------------:|---------:|---------:|---------------:|
| `brew list`   | 435.2 ± 7.2 |    428.5 |    453.1 | 329.88 ± 27.27 |
| `brasse list` |   1.3 ± 0.1 |      1.2 |      2.9 |           1.00 |

## Name

_brasse_ (pronounced \[bʁas\]) is a present and imperative form of the French
verb _brasser_, which means ‘to brew’.

## Licence

Copyright © 2022 [Nikita Karamov]\
Licensed under the [BSD 2-Clause "Simplified" License].

---

This project is hosted on GitHub: <https://github.com/kytta/brasse.git>

[bsd 2-clause "simplified" license]: https://spdx.org/licenses/BSD-2-Clause.html
[homebrew]: https://brew.sh/
[hyperfine]: https://github.com/sharkdp/hyperfine
[nikita karamov]: https://www.kytta.dev/
