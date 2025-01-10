# hexcat
`hexcat`: A minimalist CLI tool for viewing binary data in hex and ASCII.

## Usage

`hexcat` reads and displays the content of a file if the path to the file is given as the first argument. Otherwise, it reads from `stdin` instead.

### Example (with file input):

```
> hexcat src/main.rs
00000000: 75 73 65 20|73 74 64 3a|3a 66 6d 74|3a 3a 57 72| use std::fmt::Wr
00000010: 69 74 65 20|61 73 20 46|6d 74 57 72|69 74 65 3b| ite as FmtWrite;
00000020: 0a 75 73 65|20 73 74 64|3a 3a 69 6f|3a 3a 52 65| .use std::io::Re
00000030: 61 64 3b 0a|75 73 65 20|73 74 64 3a|3a 69 6f 3a| ad;.use std::io:
00000040: 3a 57 72 69|74 65 3b 0a|0a 63 6f 6e|73 74 20 47| :Write;..const G
[...]
```

### Example (with `stdin` input):

```
> echo "Hello World from hexcat" | hexcat
00000000: 48 65 6c 6c|6f 20 57 6f|72 6c 64 20|66 72 6f 6d| Hello World from
00000010: 20 68 65 78|63 61 74 0a|           |           |  hexcat.
```

## Installation

Use `cargo` to install `hexcat`:

```
cargo install --git https://github.com/mschrempf/hexcat
```
