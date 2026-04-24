
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

# Cheaty cases with suffixes, even as length is multiple of two
(дис-квалифицирован + квалифицированные) = (overlap 28 • even • ns)

# Uneven but overlaps, common in sufixes and prefixes
(历史纪录 + 纪录片) = (overlap 2 • not even • not symmetric)
```

Language data provided by [`pypi/wordfreq`](https://pypi.org/project/wordfreq/) by [@rspeer](https://github.com/rspeer) in a bundled `.txt.gz` file for:

- **Europe**: German, Greek, Spanish, French, Italian, Dutch, Polish
- **Asia**: Japanese, Korean, Chinese, Russiam
- **Americas**: English, Portuguese

## 📦 Usage

> [!NOTE]
> This is a minor project, the code and cli are your best friend on documentation.

Quick instructions on running it:

```sh
$ git clone https://github.com/Tremeschin/Shadesign && cd Shadesign
$ rustup default stable

# See options with
$ cargo run --release -- crunch --help

# For example, to find the almighty shades design
$ cargo run --release -- crunch --word design wordfreq english
```

<sup><b>Warn:</b> The code <em>will</em> use all available CPU compute power</sup>

## 📦 Results

Todo, large blobs or where

## 🔵 License

Follows the [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) terms.

Results may not be trademarked due their limited supply and uniqueness (unless you found them by yourself), and code cannot be integrated (modified or rewritten) without permission. Strictly personal or educational use is allowed, as the source code is public.

Dataset origins may be risky (scrapped, profanity, etc), I do not take any liability or warranty.

This project is purely for fun.
