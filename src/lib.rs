use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.1")
        .author("Ken Fang <kenfang@deva9.com>")
        .about("Rust cat")
        .arg(
            Arg::new("file_name")
                .allow_invalid_utf8(true)
                .value_name("FILENAME")
                .help("Input filename")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblanklines"),
        )
        .arg(
            Arg::new("number_nonblanklines")
                .short('b')
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file_name").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblanklines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut last_num = 0;
                for (line_num, line) in reader.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
