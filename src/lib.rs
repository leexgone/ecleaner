mod version;
mod plugin;

use std::{collections::HashMap, error::Error, io::ErrorKind, path::{Path, PathBuf}};
use std::fmt::Display;

use clap::{App, Arg};

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
    let plugin_set = PluginSet::new(&config.dir, config.verbose)?;
    let duplicates = plugin_set.find_duplicates();

    if duplicates.is_empty() {
        println!("There are no duplidated plugins.")
    } else {
        PluginSet::print_dupicates(&duplicates);
        if !config.test {
            PluginSet::remove_duplicates(&duplicates, &config.backup)?;
        }
    }

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

#[derive(Debug)]
struct PluginSet {
    plugins: HashMap<String, Vec<Plugin>>,
}

impl PluginSet {
    fn new(dir: &str, verbose: bool) -> Result<PluginSet, Box<dyn Error>> {
        let plugin_path = PathBuf::from(format!("{}/plugins", dir));
        if !plugin_path.is_dir() { 
            let e = std::io::Error::new(ErrorKind::NotFound, format!("Can not find `plugins` dir under `{}` dir", dir));
            return Err(Box::new(e));
        }
    
        let mut plugins: HashMap<String, Vec<Plugin>> = HashMap::new();

        log!(verbose, "Search plugins under dir `{}`...", plugin_path.display());
        for entry in plugin_path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let plugin = Plugin::new(path)?;
    
            log!(verbose, ">> {}", plugin);
            if let Some(list) = plugins.get_mut(&plugin.name) {
                list.push(plugin);
            } else {
                plugins.insert(plugin.name.clone(), vec![plugin]);
            }
        }

        for list in plugins.values_mut() {
            list.sort_by(|a, b| a.version.cmp(&b.version));
        }
    
        Ok(PluginSet { plugins })
    }

    fn find_duplicates(&self) -> Vec<&Vec<Plugin>> {
        self.plugins.values().filter(|list| list.len() > 1).collect()
    }

    fn print_dupicates(duplicates: &Vec<&Vec<Plugin>>) {
        println!("{} duplicated plugins found:", duplicates.len());
        for (i, list) in duplicates.iter().enumerate() {
            let id = i + 1;
            let plugins = *list;

            let keep = plugins.last().unwrap();
            print!("  {}\t{} [KEEP: {}; DISCARD: ", id, keep.name, keep.version);

            for (p, plugin) in plugins.iter().enumerate() {
                if p == plugins.len() - 1 {
                    break;
                }

                if p > 0 {
                    print!(", ");
                }

                print!("{}", plugin.version);
            }
            println!("]");
        }
    }

    fn remove_duplicates(duplicates: &Vec<&Vec<Plugin>>, backup: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}