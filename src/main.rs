#[macro_use]
extern crate clap;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::Path;

mod args;
mod errors;

use args::Config;

fn main() {
    match args::parse_command_line() {
        Ok(config) => {
            for filename in &config.filenames {
                println!("Hex dump of {}", filename);
                if let Err(e) = hex_dump_file(Path::new(&filename), &config) {
                    eprintln!("Failed to process '{}'.\nReason: {}", filename, e);
                }
            }
        }
        Err(e) => eprintln!("Error parsing command line: {}", e),
    }
}

/// Prints a hex dump of the given file. How the hex dump is printed is governed by the configs.
fn hex_dump_file(filename: &Path, config: &Config) -> io::Result<()> {
    // How does the user want us to print the offset?
    let print_offset: fn(u64) = match (config.uppercase, config.octal, config.decimal) {
        (true, false, false) => |n| print!("{:08X}:", n),
        (_, true, false) => |n| print!("{:12o}:", n),
        (_, false, true) => |n| print!("{:10}:", n),
        (false, false, false) | _ => |n| print!("{:08x}:", n),
    };

    let mut file = BufReader::new(File::open(filename)?);

    // Skip over 'skip' bytes.
    file.seek(SeekFrom::Start(config.skip as u64))?;

    // Limit ourselves to 'length' bytes.
    let mut file = file.take(config.length as u64);

    let mut buffer = vec![0; config.width];
    let mut offset = config.skip;
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(len) => {
                print_offset(offset);
                hex_dump_line(
                    &buffer[..len],
                    buffer.len(),
                    config.uppercase,
                    config.octal,
                    config.decimal,
                );
                offset += len as u64;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Prints a hex dump of a line supplied in a buffer.
fn hex_dump_line(buffer: &[u8], max_size: usize, uppercase: bool, octal: bool, decimal: bool) {
    // How does the user want us to print each byte?
    let print_byte: fn(u8) = match (uppercase, octal, decimal) {
        (true, false, false) => |n| print!("{:02X} ", n),
        (_, true, false) => |n| print!("{:03o} ", n),
        (_, false, true) => |n| print!("{:03} ", n),
        (false, false, false) | _ => |n| print!("{:02x} ", n),
    };

    // Print the buffer as numbers in the requested format, inserting an extra space before every 8th number.
    for i in 0..max_size {
        if (i & 7) == 0 {
            print!(" ");
        }
        if i < buffer.len() {
            let b = buffer[i];
            print_byte(b);
        } else {
            print!("   ");
        }
    }
    print!(" ");

    // Print the buffer as ASCII, replacing non-printable characters with a '.'.
    for b in buffer {
        print!(
            "{}",
            if *b >= 32 && *b < 127 {
                *b as char
            } else {
                '.'
            }
        );
    }
    println!();
}
