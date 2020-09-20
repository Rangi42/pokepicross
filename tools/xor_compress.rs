use std::env::args;
use std::process::exit;
use std::fs::{File, write};
use std::io::{Read, Error};

const PROGRAM_NAME: &str = "xor-compress";

fn compress_files<'a>(in_filenames: &'a [String], out_filename: &'a String) -> Result<u32, (&'a String, Error)> {
    let mut data = Vec::new();
    for filename in in_filenames {
        (|| {
            Ok(File::open(filename)?.read_to_end(&mut data)?)
        })().map_err(|err| (filename, err))?;
    }

    let n = data.len();

    let mut output = Vec::new();
    let mut v = 0x00;
    let mut i = 0;
    let mut runs = 0;

    while i < n {
        let mut byte = data[i];
        i += 1;
        runs += 1;

        if i == n || data[i] != v {
            // Sequential (< 0x80)
            // Run stops at 0x80 bytes or when the value two ahead is equal to v
            let mut buffer = vec![v ^ byte];
            while i < n {
                v = byte;
                if buffer.len() > 0x7f || (i + 1 < n && data[i + 1] == v) {
                    break;
                }
                byte = data[i];
                buffer.push(v ^ byte);
                i += 1;
            }
            output.push((buffer.len() - 1) as u8);
            output.extend(buffer);
        } else {
            // Alternating (>= 0x80)
            // Run stops at 0x80 bytes or when the values stop alternating
            let mut size = 0;
            while i < n && size < 0x80 && data[i] == (if size % 2 == 0 { v } else { byte }) {
                size += 1;
                i += 1;
            }
            output.push(size + 0x7f);
            output.push(v ^ byte);
            if size % 2 == 0 {
                v = byte;
            }
        }
    }

    match write(out_filename, &output[..]) {
        Ok(()) => Ok(runs),
        Err(err) => Err((out_filename, err)),
    }
}

fn main() {
    let mut argv: Vec<String> = args().skip(1).collect();

    let verbose = !argv.is_empty() && argv[0] == "-v";
    if verbose {
        argv.remove(0);
    }

    if argv.len() < 2 {
        eprintln!("Usage: {} [-v] file... files.xor", PROGRAM_NAME);
        exit(1);
    }

    let out_filename = argv.pop().unwrap();
    match compress_files(&argv[..], &out_filename) {
        Ok(runs) => if verbose {
            println!("{}: {}: ld bc, ${:x}", PROGRAM_NAME, out_filename, runs);
        },
        Err((filename, err)) => {
            eprintln!("{}: {}: {}", PROGRAM_NAME, filename, err);
            exit(err.raw_os_error().unwrap_or(1));
        }
    }
}
