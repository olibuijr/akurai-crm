#![forbid(unsafe_code)]

mod cmd_seed;
mod cmd_serve;

use std::path::PathBuf;
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("error: {msg}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some("serve") => {
            let cfg = parse_serve(&args[1..])?;
            cmd_serve::run(cfg).map_err(|e| e.to_string())
        }
        Some("seed") => {
            let db_path = args.get(1).map(|s| s.as_str()).unwrap_or("crm.db");
            cmd_seed::run(db_path)
        }
        Some("version") | Some("--version") | Some("-V") => {
            println!("akurai-crm {VERSION}");
            Ok(())
        }
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown command '{other}' (try: akurai-crm help)")),
    }
}

fn parse_serve(args: &[String]) -> Result<cmd_serve::Config, String> {
    let mut host = "127.0.0.1".to_string();
    let mut port: u16 = 8091;
    let mut dir = PathBuf::from("site/frontend");
    let mut db = "crm.db".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--host" => {
                host = next(args, &mut i, "--host")?;
            }
            "--port" | "-p" => {
                port = next(args, &mut i, "--port")?
                    .parse()
                    .map_err(|_| "invalid --port".to_string())?;
            }
            "--dir" => {
                dir = PathBuf::from(next(args, &mut i, "--dir")?);
            }
            "--db" => {
                db = next(args, &mut i, "--db")?;
            }
            other => return Err(format!("unknown flag '{other}'")),
        }
        i += 1;
    }
    Ok(cmd_serve::Config {
        host,
        port,
        dir,
        db,
    })
}

fn next(args: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    *i += 1;
    args.get(*i)
        .cloned()
        .ok_or_else(|| format!("{flag} needs a value"))
}

fn print_help() {
    println!(
        "akurai-crm {VERSION} — Pure Rust CRM\n\n\
         USAGE:\n\
         \x20 akurai-crm serve [opts]    Start the CRM web server\n\
         \x20 akurai-crm seed [db]       Seed the database with demo data\n\
         \x20 akurai-crm version         Print version\n\n\
         SERVE OPTIONS:\n\
         \x20 --host <addr>   Bind host (default: 127.0.0.1)\n\
         \x20 --port, -p <n>  Bind port (default: 8091)\n\
         \x20 --dir <path>    Frontend directory (default: site/frontend)\n\
         \x20 --db <path>     Database file (default: crm.db)"
    );
}
