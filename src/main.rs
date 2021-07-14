// TODO: rewrite filetime
extern crate filetime;

use filetime::{set_file_times, FileTime};
use std::env;
use std::fs::File;
use std::io::{stderr, stdout};
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

const MAN_PAGE: &'static str = /* @MANSTART{touch} */ r#"
NAME
    touch - create file(s)

SYNOPSIS
    touch [ -h | --help ] FILE...

DESCRIPTION
    Create the FILE(s) arguments provided

OPTIONS
    -h
    --help
        display this help and exit
"#; /* @MANEND */

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stderr = stderr();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            let _ = writeln!(stderr, "{}", "Please provide an argument");
            exit(1);
        }
        _ => match args[1].as_str() {
            "-h" | "--help" => {
                let _ = writeln!(stdout, "{}", MAN_PAGE);
                exit(0);
            }
            _ => {
                // TODO: update date/time on touch
                for arg in &args[1..] {
                    if Path::new(&arg).is_file() {
                        let time = FileTime::from_system_time(SystemTime::now());
                        if let Err(e) = set_file_times(&arg, time, time) {
                            let _ = writeln!(stderr, "{}", e);
                        }
                    } else {
                        if let Err(e) = File::create(&arg) {
                            let _ = writeln!(stderr, "{}", e);
                        }
                    }
                }
            }
        },
    };
}
