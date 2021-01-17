use std::{error::Error, fmt::Display, io::ErrorKind, path::PathBuf};

use regex::Regex;

use super::version::Version;

#[derive(Debug)]
pub struct Plugin {
    path: PathBuf,
    name: String,
    version: Version,
}

impl Display for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.version)
    }
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
    pub fn new(path: PathBuf) -> Result<Plugin, Box<dyn Error>> {
        let filename: String;
        let name = if path.is_file() {
            path.file_stem()
        } else {
            path.file_name()
        };

        match name {
            Some(stem) => {
                filename = String::from(stem.to_str().unwrap());
            },
            None => {
                return_err!(format!("Error parsing plugin: {}", path.display()));
            }
        }

        let regex = Regex::new("_\\d+[.]\\d+[.]\\d+")?;
        let (name, version) = if let Some(m) = regex.find(&filename) { 
            let plugin_name = &filename[0..m.start()];
            let version_expr = &filename[m.start() + 1..];
            match Version::parse(version_expr) {
                Ok(version) => {
                    (
                        String::from(plugin_name),
                        version,
                    )
                },
                Err(e) => {
                    return_err!(format!("Error parsings plugin `{}`: {}", path.display(), e));
                }
            }
        } else {
            (
                filename, 
                Version::new(0, 0, 0, None)
            )
        };

        Ok(Plugin {
            path,
            name,
            version
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::version::Version;

    use super::Plugin;

    #[test]
    fn test_parse_plugin_dir() {
        let path: PathBuf = PathBuf::from("D:\\eclipse\\plugins\\javax.xml.rpc_1.1.0.v201209140446");
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("javax.xml.rpc", &plugin.name);
        assert_eq!(Version::parse("1.1.0.v201209140446").unwrap(), plugin.version);
    }

    #[test]
    fn test_parse_plugin_file() {
        let path: PathBuf = PathBuf::from("D:\\eclipse\\plugins\\org.apache.commons.codec_1.13.0.v20200108-0001.jar");
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("org.apache.commons.codec", &plugin.name);
        assert_eq!(Version::parse("1.13.0.v20200108-0001").unwrap(), plugin.version);
    }

    #[test]
    fn test_parse_plugin_dual() {
        let path = PathBuf::from("d:/eclipse\\plugins\\org.w3c.dom.events_3.0.0.draft20060413_v201105210656.jar");
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("org.w3c.dom.events", &plugin.name);
        assert_eq!(Version::parse("3.0.0.draft20060413_v201105210656").unwrap(), plugin.version);
    }

    #[test]
    fn test_parse_plugin_err01() {
        let path = PathBuf::from("d:/eclipse/plugins/org.eclipse.cdt.core.win32.x86_64_6.0.0.202008310002");
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("org.eclipse.cdt.core.win32.x86_64", &plugin.name);
        assert_eq!(Version::parse("6.0.0.202008310002").unwrap(), plugin.version);
    }
}