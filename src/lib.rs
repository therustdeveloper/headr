use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

type ProgResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    author = "William Munoz",
    version,
    about = "Incomplete `GNU head` in Rust for learning purposes"
)]
pub struct Config {
    /// files to head
    #[arg(name = "FILES", default_value = "-")]
    files: Vec<String>,
    /// number of lines
    #[arg(short = 'n', long = "number", default_value = "10")]
    lines: usize,
    /// bytes to head
    #[arg(short = 'c', long = "bytes", conflicts_with = "lines" )]
    bytes: Option<usize>,
}

pub fn get_args() -> ProgResult<Config> {
    let config = Config::parse();

    Ok(config)
}

pub fn run(config: Config) -> ProgResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0  { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            },
        }
    }
    Ok(())
}

fn _parse_positive_int(val: &str) -> ProgResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> ProgResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    // 7 is an OK integer
    let res = _parse_positive_int("7");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 7);

    // any string is an error
    let res = _parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // a zero is an error
    let res = _parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
