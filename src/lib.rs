mod version;
mod plugin;

use std::{error::Error, io::ErrorKind, path::{Path, PathBuf}};
use std::fmt::Display;

use clap::{App, Arg};

use version::Version;
use plugin::Plugin;

pub struct Config {
    dir: String,
    backup: String,
    verbose: bool,
    test: bool,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[dir = {}, backup = {}, verbose = {}, test = {}]", self.dir, self.backup, self.verbose, self.test)
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
                        .arg(Arg::with_name("test")
                            .short("t")
                            .long("test")
                            .help("Scan and find the duplicated plugins, but do nothing"))   
                        .get_matches();

        let dir = matches.value_of("DIR").unwrap();
        let backup = matches.value_of("BACKUP").unwrap();
        let verbose = matches.is_present("verbose");
        let test = matches.is_present("test");

        let root_path = Path::new(&dir);
        if !root_path.is_dir() {
            let msg = format!("DIR '{}' does not exist", dir);
            return Err(msg);
        }

        let backup_path = Path::new(&backup);
        if !backup_path.is_dir() {
            let msg = format!("BACKUP dir '{}' does not exist", backup);
            return Err(msg);
        }

        Ok(Config {
            dir: String::from(dir),
            backup: String::from(backup),
            verbose,
            test
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    scan_plugins(&config.dir, config.verbose)?;

    Ok(())
}

macro_rules! log {
    ($enabled:expr) => {
        {if $enabled { println!(); }}
    };
    ($enabled:expr, $($arg:tt)*) => {
        {if $enabled { println!($($arg)*); }}
    };
}

fn scan_plugins(dir: &str, verbose: bool) -> Result<(), Box<dyn Error>> {
    let plugin_path: PathBuf = [dir, "plugins"].iter().collect();
    if !plugin_path.is_dir() { 
        let e = std::io::Error::new(ErrorKind::NotFound, format!("Can not find `plugins` dir under `{}` dir", dir));
        return Err(Box::new(e));
    }

    log!(verbose, "Search plugins under dir `{}`...", plugin_path.display());
    for entry in plugin_path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            log!(verbose, ">> {:?}", path);
        }
    }
    
    Ok(())
}