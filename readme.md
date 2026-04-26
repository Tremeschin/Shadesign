
<img align="right" height="220" src="https://github.com/user-attachments/assets/efa4b5fd-5e9b-465b-b233-449b3a8ffaa7" />
    
Some of you might know a certain.. [r/dontdeadopeninside](https://www.reddit.com/r/dontdeadopeninside/comments/8yn89x/what_a_genius/) post about an ingenious brand name seen at the right. Naturally, the next question is:

> _What are **all** great overlaps in English?_
> 
> _What about other languages?_

Having that idea on my head for years, it was time to figure it out.

<br>
<br>

## 📦 Description

This repository is a fast cruncher for word overlaps written in Rust.

Working through an example is easier to understand:

```sh
# Simpler and base case, symmetric is half word overlap
(sha-des + des-ign) = (overlap 3 • even •     symmetric)
(de-sign + sign-al) = (overlap 4 • even • not symmetric)
```

```sh
# Cheaty cases with suffixes, even as length is multiple of two
(дис-квалифицирован + квалифицированные) = (overlap 28 • even)
```

```sh
# Uneven but overlaps, common in sufixes and prefixes
(历史-纪录 + 纪录-片) = (overlap 2 • not even • not symmetric)
```

Language data is provided by [`pypi/wordfreq`](https://pypi.org/project/wordfreq/) by [@rspeer](https://github.com/rspeer) in a bundled `.txt.gz` file for:

- **Europe**: German, Greek, Spanish, French, Italian, Dutch, Polish
- **Asia**: Japanese, Korean, Chinese, Russiam
- **Americas**: English, Portuguese

<sup><b>Warn:</b> There might be some 'bad words' in the dataset, naturally.</sup>

## 📦 Usage

> [!NOTE]
> This is a minor project, code and cli are your best shot at documentation.

Quick instructions on running it:

```sh
# Ensure you have git, rustup installed
$ git clone https://github.com/Tremeschin/Shadesign && cd Shadesign
$ rustup default stable

# See options with
$ cargo run --release -- crunch --help

# For example, find all english matches
$ cargo run --release -- crunch wordfreq english

# Find the almighty shades design
$ cargo run --release -- crunch --word design wordfreq english
```

### Format

Values are printed as [serde](https://crates.io/crates/serde) deserialized json formats on each line. As such, the file format is a "JSON Lines" (`.jsonl`) - a valid and standalone json per line. It can easily be redirected to an output file:

```sh
# Lines: {"A":"although","B":"thought","overlap":6,"even":false,"symmetric":false}
$ cargo run --release -- crunch wordfreq english > english.jsonl
```

You can then filter it with tools like [`jq`](https://github.com/jqlang/jq) or [`jsongrep`](https://crates.io/crates/jsongrep):

```sh
# Filter all 'symmetric' scores
$ jq -c 'select(.symmetric)' ./english.jsonl > ./english-symmetric.jsonl
```

<b>Note:</b> Output size can be quite large depending on inputs amount.

### Performance

The code is mostly memory throughput and/or CPU L-cache bottlenecked, fully utilizing all available cores. Should take about ~90 seconds on an Ryzen 5 7520U + DDR5 5500 MT/s system, or 15 seconds in Ryzen 9 5900x + DDR4 3200 MT/s for English 50k words crunch.

However, do note the algorithm **scales in O(N²) input size**, as word order matters!

## 🔵 License

Follows the [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) terms.

Results may not be trademarked due their limited supply and uniqueness (unless you found them by yourself), and code cannot be integrated (modified or rewritten) without permission. Strictly personal or educational use is allowed, as the source code is public. Dataset origins may be risky (scrapped, profanity, etc), I do not take any liability or warranty. This project is purely for fun.
