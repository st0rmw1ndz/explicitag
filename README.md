# explicitag

[![Rust](https://github.com/st0rmw1ndz/explicitag/workflows/Rust/badge.svg)](https://github.com/st0rmw1ndz/explicitag/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/releases/latest)
[![License](https://img.shields.io/github/license/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/blob/main/LICENSE)

An automatic MP4 rating tagger based on embedded or searched lyrics.

## Usage

Clone the repository and build, or use `cargo run`.

```
An automatic MP4 rating tagger based on embedded or searched lyrics.

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

<details>
  <summary>Expand to see example</summary>
  <br>

  ```
  D:\Files\Music Library\Gorillaz\(2010) Plastic Beach\1.09 - Some Kind of Nature.m4a - Rating is the same as current
  D:\Files\Music Library\Gorillaz\(2010) Plastic Beach\1.10 - On Melancholy Hill.m4a - Rating is the same as current
  D:\Files\Music Library\Motion City Soundtrack\(2003) I Am the Movie\1.01 - Cambridge.m4a - Rating is the same as current
  D:\Files\Music Library\Gorillaz\(2010) Plastic Beach\1.05 - Stylo.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.09 - November Has Come.m4a - (Local) Explicit
  D:\Files\Music Library\Motion City Soundtrack\(2005) Commit This to Memory\1.03 - When _You're_ Around.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.15 - Demon Days.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2010) Plastic Beach\1.03 - White Flag.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2001) Gorillaz\1.01 - Re-Hash.m4a - (Local) Explicit
  D:\Files\Music Library\Motion City Soundtrack\(2003) I Am the Movie\1.13 - Autographs & Apologies.m4a - (Local) Explicit
  D:\Files\Music Library\Modest Mouse\(2007) We Were Dead Before the Ship Even Sank\1.13 - People as Places as People.m4a - (Local) Explicit
  D:\Files\Music Library\Motion City Soundtrack\(2005) Commit This to Memory\1.04 - Resolution.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.06 - Feel Good Inc..m4a - (Local) Explicit
  D:\Files\Music Library\Modest Mouse\(2007) We Were Dead Before the Ship Even Sank\1.14 - Invisible.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2001) Gorillaz\1.15 - M1 A1.m4a - (Local) Explicit
  D:\Files\Music Library\Filter\(2002) The Amalgamut\1.12 - The 4th.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.14 - Don’t Get Lost in Heaven.m4a - Rating is the same as current
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.01 - Intro.m4a - Rating is the same as current
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.11 - White Light.m4a - Rating is the same as current
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.12 - DARE.m4a - Rating is the same as current
  D:\Files\Music Library\Motion City Soundtrack\(2005) Commit This to Memory\1.05 - Feel Like Rain.m4a - Rating is the same as current
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.13 - Fire Coming Out of the Monkey’s Head.m4a - (Local) Explicit
  D:\Files\Music Library\Motion City Soundtrack\(2003) I Am the Movie\1.02 - Shiver.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.07 - El Mañana.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.08 - Every Planet We Reach Is Dead.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.10 - All Alone.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2010) Plastic Beach\1.04 - Rhinestone Eyes.m4a - (Local) Explicit
  D:\Files\Music Library\Gorillaz\(2005) Demon Days\1.05 - Dirty Harry.m4a - (Local) Explicit

  ```
</details>

## Future Features

- [ ] Support a custom words list file as an argument.
- [ ] Properly address API ratelimits.
- [ ] Improve IO operations speed somehow.

## Attributions

- [Lyrist](https://lyrist.vercel.app) - Lyrics API
- [List of Dirty, Naughty, Obscene, and Otherwise Bad Words](https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words) - Explicit words list