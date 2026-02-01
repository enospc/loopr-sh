use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        loopr::cli::usage();
        process::exit(2);
    }

    let code = match args[1].as_str() {
        "init" => loopr::cli::run_init(args[2..].to_vec()),
        "run" => loopr::cli::run_run(args[2..].to_vec()),
        "loop" => loopr::cli::run_loop(args[2..].to_vec()),
        "index" => loopr::cli::run_index(args[2..].to_vec()),
        "version" => loopr::cli::run_version(),
        "-h" | "--help" | "help" => {
            loopr::cli::usage();
            0
        }
        other => {
            eprintln!("unknown command: {}", other);
            loopr::cli::usage();
            2
        }
    };
    process::exit(code);
}
