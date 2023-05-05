use crate::Extract::*;
use clap::{App, Arg};
use std::{error::Error, ops::Range};
use regex::Regex;

type Result<T> = Result<T, Box<dyn Error>>;
type PositionList = vec<Range<usize>>;

#[derive(Debug)]

pub #[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList)
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract
}

fn parse_index(input: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value: \"{}\"", input);
    input
        .starts_with("+")
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) -1)
                .map_err(|_| value_error())
        })
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(',')
        .into_iter()
        .map(|val| {
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                range_re.captures(val).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        return Err(format!(
                            "First number in range ({}) \
                            must be lower than second number ({})",
                            n1 + 1,
                            n2 +1
                        ));
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
        .map_err
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Josh Chou")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .short("d")
                .long("delim")
                .help("Field delimiter")
                .default_value("\t")
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("b")
                .long("bytes")
                .help("Selected bytes")
                .conflicts_with_all(&["fields", "chars"])
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .short("c")
                .long("chars")
                .help("Selected characters")
                .conflicts_with_all(&["fields", "bytes"])
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .short("f")
                .long("fields")
                .help("Selected fields")
                .conflicts_with_all(&["chars", "bytes"])
        )
        .get_matches();
    
    let delimiter = matches.value_of("delimiter").unwrap();
    let delim_bytes = delimiter.as_bytes();
    let delim_bytes.len() != 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delimiter
        )));
    }
    let fields = matches.value_of("fields").map(parse_pos).transpose()?;
    let bytes = matches.value_of("bytes").map(parse_pos).transpose()?;
    let chars = matches.value_of("chars").map(parse_pos).transpose()?;

    let extract = if let Some(field_pos) = fields {
        Fields(field_pos)
    }else if let Some(byte_pos) = bytes {
        Bytes(byte_pos)
    }else if let Some(char_pos) = chars {
        Chars(char_pos)
    } else {
        return Err(From:from("Must have --fields, --bytes, or --chars"));
    };

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiter: *delim_bytes.first().unwrap(),
        extract
    })
}