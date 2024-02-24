# explicitag

[![Rust](https://github.com/st0rmw1ndz/explicitag/workflows/Rust/badge.svg)](https://github.com/st0rmw1ndz/explicitag/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/releases/latest)
[![License](https://img.shields.io/github/license/st0rmw1ndz/explicitag)](https://github.com/st0rmw1ndz/explicitag/blob/main/LICENSE)

An automatic MP4 rating tagger based on embedded or searched lyrics.

## Usage

Download the latest release, or use `cargo run` on the project.

```
An automatic MP4 rating tagger based on embedded or searched lyrics.

Usage: explicitag [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Files or directories to process

Options:
  -c, --mark-clean    Mark safe tracks as specifically clean
  -a, --use-api       Search API for lyrics if local ones aren't present
  -w, --write-lyrics  Write lyrics from the API to files
  -n, --no-write      Disable writing lyrics and ratings to files
  -q, --quiet         Suppress all output from the program
  -h, --help          Print help
  -V, --version       Print version

https://github.com/st0rm1wndz/explicitag
```

> [!NOTE]
> The lyrics being searched via the API is an opt-in feature with the `--use-api` flag.

## Example

<details>
  <summary>Expand to see example</summary>
  <br>

  ```
[2024-02-23T12:00:05Z INFO  explicitag] Running with 403 words marked as explicit.
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.01 - Frank Sinatra.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.02 - The Distance.m4a - (Local) Explicit
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.03 - Friend Is a Four Letter Word.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.04 - Open Book.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.05 - Daria.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.06 - Race Car Ya‐Yas.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.07 - I Will Survive.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.08 - Stickshifts and Safetybelts.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.09 - Perhaps, Perhaps, Perhaps.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.10 - It’s Coming Down.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.11 - Nugget.m4a - (Local) Explicit
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.12 - She’ll Come Back to Me.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.13 - Italian Leather Sofa.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1996) Fashion Nugget\1.14 - Sad Songs and Waltzes.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.01 - Satan Is My Motor.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.02 - Mexico.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.03 - Never There.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.04 - Guitar.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.05 - You Turn the Screws.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.06 - Walk on By.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.07 - Sheep Go to Heaven.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.08 - When You Sleep.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.09 - Hem of Your Garment.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.10 - Alpha Beta Parking Lot.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.11 - Let Me Go.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.12 - Cool Blue Reason.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(1998) Prolonging the Magic\1.13 - Where Would I Be_.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.01 - Opera Singer.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.02 - Meanwhile, Rick James….m4a - (Local) Explicit
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.03 - Shadow Stabbing.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.04 - Short Skirt_Long Jacket.m4a - (Local) Explicit
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.05 - Commissioning a Symphony in C.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.06 - Arco Arena.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.07 - Comfort Eagle.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.08 - Long Line of Cars.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.09 - Love You Madly.m4a - (Local) Explicit
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.10 - Pretty Pink Ribbon.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] D:\Files\Music Library\CAKE\(2001) Comfort Eagle\1.11 - World of Two.m4a - (Local) Inoffensive
[2024-02-23T12:00:05Z INFO  explicitag] Processed 38 files. Skipped 0 files.
  ```
</details>

## Future Features

- [ ] Allow customization of log output.
- [ ] Support a custom words list file as an argument.
- [ ] Properly address API ratelimits.
- [x] Improve IO operations speed somehow.

## Attributions

- [Lyrist](https://lyrist.vercel.app) - Lyrics API
- [List of Dirty, Naughty, Obscene, and Otherwise Bad Words](https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words) - Explicit words list