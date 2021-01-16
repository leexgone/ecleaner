use std::{error::Error, io::ErrorKind, path::PathBuf};

use super::version::Version;

pub struct Plugin {
    path: PathBuf,
    name: String,
    version: Version,
}

macro_rules! return_err {
    ($msg:expr) => {
        {
            let e = std::io::Error::new(ErrorKind::Other, $msg);
            return Err(Box::new(e));
        }
    }
}

impl Plugin {
    fn new(path: PathBuf) -> Result<Plugin, Box<dyn Error>> {
        let filename: String;
        match path.file_stem() {
            Some(stem) => {
                filename = String::from(stem.to_str().unwrap());
            },
            None => {
                // let e = std::io::Error::new(ErrorKind::Other, format!("Error parsing plugin: {}", path.display()));
                // return Err(Box::new(e));
                return_err!(format!("Error parsing plugin: {}", path.display()));
            }
        }

        let p;
        if let Some(pos) = filename.rfind('_') {
            p = pos;
        } else {
            return_err!(format!("Error parsing plugin name: {}", filename))
        }

        let (plugin_name, version_expr) = filename.split_at(p);
        let version_expr = &version_expr[1..];

        let version = Version::parse(version_expr)?;

        Ok(Plugin {
            path,
            name: String::from(plugin_name),
            version
        })
    }
}