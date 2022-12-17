# brasse

`brasse` is a [Homebrew] clone developed with Go. Goals of this project are:

- increase speeds for operations performed on the machine
  - `brew list` takes 1.3 seconds
  - `brew search git` takes 8.7 seconds
  - `brew info git` takes 1.4 seconds
- provide good piping support to allow for, for example:
  - pipe a list of package names to `install`
  - pipe a list from `outdated` to `info`
- (optionally) provide better dependency managed
  - similarly to APT, differentiate automatically and manually installed
    packages to remove the former if they have no dependents
- keep everything compatible with the official `brew` executable

## Status

This is very much an experimantal project.

Benchmarks are done with [`hyperfine`][hyperfine]

### Currently implemented

#### `brasse list`

Currently, this just outputs the list of formulae and casks, similar to `brew list` output without any arguments.

| Command       |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
| :------------ | ----------: | -------: | -------: | -----------: |
| `brew list`   | 490.6 ± 3.5 |    484.7 |    495.9 | 44.43 ± 2.21 |
| `brasse list` |  11.0 ± 0.5 |     10.0 |     12.6 |         1.00 |

## Name

_brasse_ (pronounced \[bʁas\]) is a present and imperative form of the French
verb _brasser_, which means ‘to brew’.

## Licence

Copyright © 2022 [Nikita Karamov]\
Licensed under the [BSD 2-Clause "Simplified" License].

---

This project is hosted on Codeberg: <https://codeberg.org/kytta/go-brew.git>

[bsd 2-clause "simplified" license]: https://spdx.org/licenses/BSD-2-Clause.html
[homebrew]: https://brew.sh/
[hyperfine]: https://github.com/sharkdp/hyperfine
[nikita karamov]: https://www.kytta.dev/
