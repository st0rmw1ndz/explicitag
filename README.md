# explicitag

<!-- [![Rust](https://github.com/st0rmw1ndz/explicitag/workflows/Rust/badge.svg)](https://github.com/st0rmw1ndz/explicitag/actions/workflows/rust.yml) -->
<!-- [![Release](https://img.shields.io/github/v/release/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/releases/latest) -->
[![License](https://img.shields.io/github/license/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/blob/main/LICENSE)

An automatic MP4 rating tagger based on embedded lyrics.

## Usage

```
An automatic MP4 rating tagger based on embedded lyrics.

Usage: explicitag [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Files or directories to process

Options:
  -h, --help     Print help
  -V, --version  Print version

https://github.com/st0rm1wndz/explicitag
```

## Example

```
> explicitag "D:\Files\Music Library\Tyler, the Creator\(2009) Bastard"
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.01 - Bastard.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.03 - Odd Toddlers.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.02 - Seven.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.06 - Pigs Fly.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.07 - Parade.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.09 - AssMilk.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.05 - Blow.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.04 - French!.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.12 - Sarah.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.08 - Slow It Down.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.10 - VCR_Wheels.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.11 - Session.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.13 - Jack and the Beanstalk.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.14 - Tina.m4a - Explicit
File: D:\Files\Music Library\Tyler, the Creator\(2009) Bastard\1.15 - Inglorious.m4a - Explicit
```

> ![NOTE]
> The order in which files are processed is scrambled since it's asynchronous.