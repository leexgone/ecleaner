use std::{error::Error, fmt::Display, path::Path};

use clap::{App, Arg};

pub struct Config {
    dir: String,
    backup: String,
    verbose: bool,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[dir = {}, backup = {}, verbose = {}]", self.dir, self.backup, self.verbose)
    }
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let matches = App::new("eclean")
                        .version("0.1.0")
                        .author("Steven Lee <leexgone@163.com>")
                        .about("Clean up the duplicated plugins in eclipse plugins directory.")
                        .arg(Arg::with_name("DIR")
                            .help("The eclipse root directory to be cleaned. The `/plugins` directory should be under this directory.")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("BACKUP")
                            .help("Specify a backup directory to store the removed plugins.")
                            .required(true)
                            .index(2))
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Use verbose output"))
                        .get_matches();

        let dir = matches.value_of("DIR").unwrap();
        let backup = matches.value_of("BACKUP").unwrap();
        let verbose = matches.is_present("verbose");

        let root_path = Path::new(&dir);
        if !root_path.exists() {
            let msg = format!("DIR '{}' does not exist", dir);
            return Err(msg);
        }

        let backup_path = Path::new(&backup);
        if !backup_path.exists() {
            let msg = format!("BACKUP dir '{}' does not exist", backup);
            return Err(msg);
        }

        Ok(Config {
            dir: String::from(dir),
            backup: String::from(backup),
            verbose
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config);

    Ok(())
}