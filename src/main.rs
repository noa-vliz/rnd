use std::io::{BufRead, Read, Write};
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = handle.read_line(&mut line).expect("read_line failed");

            if bytes_read == 0 {
                break;
            }

            print!("{}", line);
            io::stdout().flush().unwrap();
        }
    }

    for file_name in &args[1..] {
        match fs::File::open(&file_name) {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();

                if let Err(e) = file.read_to_end(&mut buffer) {
                    eprintln!("{}: {}: {}", &args[0], &file_name, e);
                    continue;
                };

                let stdout = io::stdout();
                let mut handle = stdout.lock();
                if let Err(e) = handle.write_all(&buffer) {
                    eprintln!("{}: {}: {}", &args[0], &file_name, e);
                    continue;
                };
            }
            Err(e) => {
                eprintln!("{}: {}: {}", &args[0], &file_name, e.kind());
                continue;
            }
        }
    }
}
