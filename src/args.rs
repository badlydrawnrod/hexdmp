use crate::errors::HexDumpError;
use clap::{App, Arg};
use std;

pub struct Config {
    pub uppercase: bool,        // Display hex numbers in uppercase.
    pub octal: bool,            // Display numbers as octal, not hex.
    pub decimal: bool,          // Display numbers as decimal, not hex.
    pub width: usize,           // The number of bytes per row.
    pub skip: u64,              // Skip n bytes before outputting.
    pub length: u64,            // Truncate output after n bytes.
    pub filenames: Vec<String>, // The filenames to dump.
}

/// Parses the command line, returning the configuration.
pub fn parse_command_line() -> Result<Config, HexDumpError> {
    // Parse the command line.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("columns")
                .help("Displays n columns of output")
                .short("c")
                .long("columns")
                .takes_value(true)
                .default_value("16")
                .possible_values(&["8", "16", "24", "32"]),
        )
        .arg(
            Arg::with_name("length")
                .help("Displays only n bytes of input [suffixes: Ki[B], Mi[B], Gi[B], Ti[B] Pi[B], K[B], M[B], G[B], T[B], P[B]]")
                .short("n")
                .long("length")
                .validator(is_valid_offset)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip")
                .help("Skips over the first n bytes of input [suffixes: Ki[B], Mi[B], Gi[B], Ti[B] Pi[B], K[B], M[B], G[B], T[B], P[B]]")
                .short("s")
                .long("skip")
                .visible_alias("start")
                .validator(is_valid_offset)
              .takes_value(true),
        )
        .arg(
            Arg::with_name("uppercase")
                .help("Displays hex values in uppercase")
                .short("u")
                .long("upper"),
        )
        .arg(
            Arg::with_name("octal")
                .help("Displays values in octal")
                .short("o")
                .long("octal")
                .overrides_with("decimal"),
        )
        .arg(
            Arg::with_name("decimal")
                .help("Displays values in decimal")
                .short("d")
                .long("decimal")
                .overrides_with("octal"),
        )
        .arg(Arg::with_name("filenames").required(true).multiple(true))
        .get_matches();

    // How many columns are there?
    let width = value_t!(matches, "columns", usize).unwrap();

    // Should hex letters should be uppercased.
    let uppercase = matches.is_present("uppercase");

    // Should output be in octal?
    let octal = matches.is_present("octal");

    // Should output be in decimal?
    let decimal = matches.is_present("decimal");

    // How many bytes should be skipped?
    let skip = match matches.value_of("skip") {
        Some(s) => from_suffixed_str(s)?,
        None => 0,
    };

    // Get the maximum length of the output.
    let length = match matches.value_of("length") {
        Some(s) => from_suffixed_str(s)?,
        None => std::u64::MAX,
    };

    // It's safe to unwrap here as <filenames>... is a required argument.
    let filenames = matches
        .values_of("filenames")
        .unwrap()
        .map(String::from)
        .collect();

    Ok(Config {
        uppercase,
        octal,
        decimal,
        width,
        skip,
        length,
        filenames,
    })
}

fn is_valid_offset(v: String) -> Result<(), String> {
    match from_suffixed_str(&v) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Expected a positive decimal, octal or hex integer and an optional suffix.\n[suffixes: Ki[B], Mi[B], Gi[B], Ti[B] Pi[B], K[B], M[B], G[B], T[B], P[B]]"))
    }
}

fn from_suffixed_str(input: &str) -> Result<u64, HexDumpError> {
    // Determine the number base from the prefix.
    let (base, input) = if input.starts_with("0x") {
        // Hexadecimal.
        (16, &input[2..])
    } else if input.starts_with('0') {
        // Octal.
        (8, &input[1..])
    } else {
        // Assume decimal.
        (10, &input[..])
    };

    // Determine the multiplier from the suffix.
    let (mult, input) = if let Some(suffix) = input.find(|c: char| !c.is_digit(base)) {
        let mult = multiplier_from_suffix(&input[suffix..])?;
        let input = &input[..suffix];
        (mult, input)
    } else {
        (1, input)
    };

    Ok(mult * u64::from_str_radix(input, base)?)
}

fn multiplier_from_suffix(suffix: &str) -> Result<u64, HexDumpError> {
    match suffix {
        // Decimal suffixes.
        "K" | "KB" => Ok(1000),
        "M" | "MB" => Ok(1000 * 1000),
        "G" | "GB" => Ok(1000 * 1000 * 1000),
        "T" | "TB" => Ok(1000 * 1000 * 1000 * 1000),
        "P" | "PB" => Ok(1000 * 1000 * 1000 * 1000 * 1000),

        // Binary suffixes.
        "Ki" | "KiB" => Ok(1024),
        "Mi" | "MiB" => Ok(1024 * 1024),
        "Gi" | "GiB" => Ok(1024 * 1024 * 1024),
        "Ti" | "TiB" => Ok(1024 * 1024 * 1024 * 1024),
        "Pi" | "PiB" => Ok(1024 * 1024 * 1024 * 1024 * 1024),

        // Anything else is an error.
        _ => Err(HexDumpError::UnknownSuffix),
    }
}
