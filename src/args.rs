use clap::{App, Arg};
use errors::HexDumpError;
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
        .map(|f| String::from(f))
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
        Err(_) => Err(String::from("Expected a positive integer and an optional suffix [suffixes: Ki[B], Mi[B], Gi[B], Ti[B], Pi[B] K[B], M[B], G[B], T[B], P[B]]"))
    }
}

fn from_suffixed_str(mut s: &str) -> Result<u64, HexDumpError> {
    if let Some(suffix) = s.find(|c: char| !c.is_digit(10)) {
        // There's a multiplicative suffix, so determine what it is.
        let mult = &s[suffix..];
        s = &s[..suffix];

        let mult: u64 = match mult {
            // Decimal suffixes.
            "K" | "KB" => 1000,
            "M" | "MB" => 1000 * 1000,
            "G" | "GB" => 1000 * 1000 * 1000,
            "T" | "TB" => 1000 * 1000 * 1000 * 1000,
            "P" | "PB" => 1000 * 1000 * 1000 * 1000 * 1000,
            // Binary suffixes.
            "Ki" | "KiB" => 1024,
            "Mi" | "MiB" => 1024 * 1024,
            "Gi" | "GiB" => 1024 * 1024 * 1024,
            "Ti" | "TiB" => 1024 * 1024 * 1024 * 1024,
            "Pi" | "PiB" => 1024 * 1024 * 1024 * 1024 * 1024,
            _ => return Err(HexDumpError::UnknownSuffix),
        };

        // Now we have the multiplier, let's use it.
        Ok(mult * u64::from_str_radix(s, 10)?)
    } else {
        // There's no multiplicative suffix.
        Ok(u64::from_str_radix(s, 10)?)
    }
}
