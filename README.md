# hexdmp
hexdmp is a simple command-line tool to display the contents of files as hexadecimal.

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).

## Installation
hexdmp is a Rust program. The easiest way to install it is to get cargo, then run the following:
```
$ cargo install hexdmp
```

## Usage
```console
$ hexdmp -h
hexdmp 0.1.0
Badly Drawn Rod <rod@badlydrawngames.com>
hexdmp is a command line utility that dumps the contents of one or more files to stdout in hex.

USAGE:
    hexdmp [FLAGS] [OPTIONS] <filenames>...

FLAGS:
    -d, --decimal    Displays values in decimal
    -h, --help       Prints help information
    -o, --octal      Displays values in octal
    -u, --upper      Displays hex values in uppercase
    -V, --version    Prints version information

OPTIONS:
    -c, --columns <columns>    Displays n columns of output [default: 16]  [possible values: 8, 16, 24, 32]
    -n, --length <length>      Displays only n bytes of input [suffixes: Ki[B], Mi[B], Gi[B], Ti[B] Pi[B], K[B], M[B],
                               G[B], T[B], P[B]]
    -s, --skip <skip>          Skips over the first n bytes of input [suffixes: Ki[B], Mi[B], Gi[B], Ti[B] Pi[B], K[B],
                               M[B], G[B], T[B], P[B]]

ARGS:
    <filenames>...
```

## Example
Display 1KiB of a file in 24-column format, skipping the first 2KiB.
```console
$ hexdmp --columns 24 --skip 2KiB --length 1KiB src/main.rs
Hex dump of src\main.rs
00000800: 6c 69 6e 65 20 73 75 70  70 6c 69 65 64 20 69 6e  20 61 20 62 75 66 66 65  line supplied in a buffe
00000818: 72 2e 0d 0a 66 6e 20 68  65 78 5f 64 75 6d 70 5f  6c 69 6e 65 28 62 75 66  r...fn hex_dump_line(buf
00000830: 66 65 72 3a 20 26 5b 75  38 5d 2c 20 6d 61 78 5f  73 69 7a 65 3a 20 75 73  fer: &[u8], max_size: us
00000848: 69 7a 65 2c 20 75 70 70  65 72 63 61 73 65 3a 20  62 6f 6f 6c 2c 20 6f 63  ize, uppercase: bool, oc
00000860: 74 61 6c 3a 20 62 6f 6f  6c 2c 20 64 65 63 69 6d  61 6c 3a 20 62 6f 6f 6c  tal: bool, decimal: bool
00000878: 29 20 7b 0d 0a 20 20 20  20 2f 2f 20 48 6f 77 20  64 6f 65 73 20 74 68 65  ) {..    // How does the
00000890: 20 75 73 65 72 20 77 61  6e 74 20 75 73 20 74 6f  20 70 72 69 6e 74 20 65   user want us to print e
000008a8: 61 63 68 20 62 79 74 65  3f 0d 0a 20 20 20 20 6c  65 74 20 70 72 69 6e 74  ach byte?..    let print
000008c0: 5f 62 79 74 65 20 3d 20  6d 61 74 63 68 20 28 75  70 70 65 72 63 61 73 65  _byte = match (uppercase
000008d8: 2c 20 6f 63 74 61 6c 2c  20 64 65 63 69 6d 61 6c  29 20 7b 0d 0a 20 20 20  , octal, decimal) {..
000008f0: 20 20 20 20 20 28 66 61  6c 73 65 2c 20 66 61 6c  73 65 2c 20 66 61 6c 73       (false, false, fals
00000908: 65 29 20 3d 3e 20 70 72  69 6e 74 5f 62 79 74 65  5f 6c 6f 77 65 72 2c 0d  e) => print_byte_lower,.
00000920: 0a 20 20 20 20 20 20 20  20 28 74 72 75 65 2c 20  66 61 6c 73 65 2c 20 66  .        (true, false, f
00000938: 61 6c 73 65 29 20 3d 3e  20 70 72 69 6e 74 5f 62  79 74 65 5f 75 70 70 65  alse) => print_byte_uppe
00000950: 72 2c 0d 0a 20 20 20 20  20 20 20 20 28 5f 2c 20  74 72 75 65 2c 20 66 61  r,..        (_, true, fa
00000968: 6c 73 65 29 20 3d 3e 20  70 72 69 6e 74 5f 62 79  74 65 5f 6f 63 74 61 6c  lse) => print_byte_octal
00000980: 2c 0d 0a 20 20 20 20 20  20 20 20 28 5f 2c 20 66  61 6c 73 65 2c 20 74 72  ,..        (_, false, tr
00000998: 75 65 29 20 3d 3e 20 70  72 69 6e 74 5f 62 79 74  65 5f 64 65 63 69 6d 61  ue) => print_byte_decima
000009b0: 6c 2c 0d 0a 20 20 20 20  20 20 20 20 5f 20 3d 3e  20 70 72 69 6e 74 5f 62  l,..        _ => print_b
000009c8: 79 74 65 5f 6c 6f 77 65  72 2c 0d 0a 20 20 20 20  7d 3b 0d 0a 0d 0a 20 20  yte_lower,..    };....
000009e0: 20 20 2f 2f 20 50 72 69  6e 74 20 74 68 65 20 62  75 66 66 65 72 20 61 73    // Print the buffer as
000009f8: 20 6e 75 6d 62 65 72 73  20 69 6e 20 74 68 65 20  72 65 71 75 65 73 74 65   numbers in the requeste
00000a10: 64 20 66 6f 72 6d 61 74  2c 20 69 6e 73 65 72 74  69 6e 67 20 61 6e 20 65  d format, inserting an e
00000a28: 78 74 72 61 20 73 70 61  63 65 20 62 65 66 6f 72  65 20 65 76 65 72 79 20  xtra space before every
00000a40: 38 74 68 20 6e 75 6d 62  65 72 2e 0d 0a 20 20 20  20 66 6f 72 20 69 20 69  8th number...    for i i
00000a58: 6e 20 30 2e 2e 6d 61 78  5f 73 69 7a 65 20 7b 0d  0a 20 20 20 20 20 20 20  n 0..max_size {..
00000a70: 20 69 66 20 28 69 20 26  20 37 29 20 3d 3d 20 30  20 7b 0d 0a 20 20 20 20   if (i & 7) == 0 {..
00000a88: 20 20 20 20 20 20 20 20  70 72 69 6e 74 21 28 22  20 22 29 3b 0d 0a 20 20          print!(" ");..
00000aa0: 20 20 20 20 20 20 7d 0d  0a 20 20 20 20 20 20 20  20 69 66 20 69 20 3c 20        }..        if i <
00000ab8: 62 75 66 66 65 72 2e 6c  65 6e 28 29 20 7b 0d 0a  20 20 20 20 20 20 20 20  buffer.len() {..
00000ad0: 20 20 20 20 6c 65 74 20  62 20 3d 20 62 75 66 66  65 72 5b 69 5d 3b 0d 0a      let b = buffer[i];..
00000ae8: 20 20 20 20 20 20 20 20  20 20 20 20 70 72 69 6e  74 5f 62 79 74 65 28 62              print_byte(b
00000b00: 29 3b 0d 0a 20 20 20 20  20 20 20 20 7d 20 65 6c  73 65 20 7b 0d 0a 20 20  );..        } else {..
00000b18: 20 20 20 20 20 20 20 20  20 20 70 72 69 6e 74 21  28 22 20 20 20 22 29 3b            print!("   ");
00000b30: 0d 0a 20 20 20 20 20 20  20 20 7d 0d 0a 20 20 20  20 7d 0d 0a 20 20 20 20  ..        }..    }..
00000b48: 70 72 69 6e 74 21 28 22  20 22 29 3b 0d 0a 0d 0a  20 20 20 20 2f 2f 20 50  print!(" ");....    // P
00000b60: 72 69 6e 74 20 74 68 65  20 62 75 66 66 65 72 20  61 73 20 41 53 43 49 49  rint the buffer as ASCII
00000b78: 2c 20 72 65 70 6c 61 63  69 6e 67 20 6e 6f 6e 2d  70 72 69 6e 74 61 62 6c  , replacing non-printabl
00000b90: 65 20 63 68 61 72 61 63  74 65 72 73 20 77 69 74  68 20 61 20 27 2e 27 2e  e characters with a '.'.
00000ba8: 0d 0a 20 20 20 20 66 6f  72 20 62 20 69 6e 20 62  75 66 66 65 72 20 7b 0d  ..    for b in buffer {.
00000bc0: 0a 20 20 20 20 20 20 20  20 70 72 69 6e 74 21 28  0d 0a 20 20 20 20 20 20  .        print!(..
00000bd8: 20 20 20 20 20 20 22 7b  7d 22 2c 0d 0a 20 20 20  20 20 20 20 20 20 20 20        "{}",..
00000bf0: 20 69 66 20 2a 62 20 3e  3d 20 33 32 20 26 26 20                            if *b >= 32 &&
```
