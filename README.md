# explicitag

<!-- [![Rust](https://github.com/st0rmw1ndz/explicitag/workflows/Rust/badge.svg)](https://github.com/st0rmw1ndz/explicitag/actions/workflows/rust.yml) -->
<!-- [![Release](https://img.shields.io/github/v/release/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/releases/latest) -->
[![License](https://img.shields.io/github/license/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/blob/main/LICENSE)

An automatic MP4 rating tagger based on embedded lyrics.

## Usage

Clone the repository and build, or use `cargo run`.

```
An automatic MP4 rating tagger based on embedded lyrics.

Usage: explicitag [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Files or directories to process

Options:
  -c, --mark-clean    Mark safe tracks as specifically clean
  -a, --use-api       Search API for lyrics if local ones aren't present
  -w, --write-lyrics  Write the lyrics from the API to the file
  -q, --quiet         Suppress all output from the program
  -h, --help          Print help
  -V, --version       Print version

https://github.com/st0rm1wndz/explicitag
```

## Example

```
> explicitag "D:\Files\Music Library\deadmau5\(2010) 4x4=12"
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.01 - Some Chords.m4a - No lyrics found
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.05 - Animal Rights.m4a - No lyrics found
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.10 - Everything Before.m4a - No lyrics found
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.03 - City in Florida.m4a - No lyrics found
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.07 - Right This Second.m4a - No lyrics found
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.09 - One Trick Pony.m4a - (Local) Explicit
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.04 - Bad Selection.m4a - (Local) Inoffensive
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.06 - I Said (Michael Woods Remix).m4a - (Local) Inoffensive
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.08 - Raise Your Weapon.m4a - (Local) Explicit
D:\Files\Music Library\deadmau5\(2010) 4x4=12\1.02 - SOFI Needs a Ladder.m4a - (Local) Inoffensive
Processed 10 files. Skipped 5 files.
```

## Future Features

- [ ] Rework the words list, it's tailored towards bypassed words.
- [ ] Support a custom words list as an argument.
- [ ] Properly address API ratelimits.