use std::{env, fs};

const HELP_MESSAGE: &str = r#"Usage: data2sound <command> <input> <output>
Commands:
    encode,     e  Encode a file to a wav file
    decode,     d  Decode a wav file to a file
Options:
    --help,    -h  Print this help message
    --version, -v  Print the version
"#;

fn version() {
    println!(
        "data2sound {} {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
    );
}

fn help() {
    println!("{}", HELP_MESSAGE);
}

fn try_main() -> data2sound::Result<()> {
    // Skip the first argument, which is the path to the executable
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--version" || arg == "-v") {
        version()
    } else if args.iter().any(|arg| arg == "--help" || arg == "-h") || args.len() < 4 {
        help()
    } else {
        let mut args = args.iter().skip(1);
        let command = args.next().unwrap();
        let input = args.next().unwrap();
        let output = args.next().unwrap();
        match command.as_str() {
            "encode" | "e" => data2sound::encode(fs::File::open(input)?, output)?,
            "decode" | "d" => data2sound::decode(input, output)?,
            _ => eprintln!(
                "Unknown command '{}' Run 'data2sound --help' for more information",
                command
            ),
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
