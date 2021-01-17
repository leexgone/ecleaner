use std::process;

use eclean::Config;

fn main() {
    let  config = Config::new().unwrap_or_else(|err| {
        eprintln!("Error when parsing arguments: {}.", err);
        process::exit(1);
    });

    if let Err(e) = eclean::run(config) {
        eprintln!("Error when cleaning up: {}", e);
        process::exit(2);
    }
}